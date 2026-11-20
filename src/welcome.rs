use std::{
    env,
    error::Error,
    io::Write,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

use crate::state::State;

const WIN_W: f64 = 960.0;
const WIN_H: f64 = 700.0;

/// If the page never reports "ready", show the window anyway after this long.
const SHOW_FALLBACK: Duration = Duration::from_millis(1200);

/// Embed the logo as a data URI (no extra crate, no file at runtime).
fn base64_encode(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut out = String::with_capacity(data.len() * 4 / 3 + 4);

    for c in data.chunks(3) {
        let (a, b, d) = (
            c[0] as usize,
            *c.get(1).unwrap_or(&0) as usize,
            *c.get(2).unwrap_or(&0) as usize,
        );

        out.push(T[a >> 2] as char);
        out.push(T[((a & 3) << 4) | (b >> 4)] as char);
        out.push(if c.len() > 1 {
            T[((b & 15) << 2) | (d >> 6)] as char
        } else {
            '='
        });
        out.push(if c.len() > 2 { T[d & 63] as char } else { '=' });
    }

    out
}

/// Called on startup. Never blocks: the welcome window runs in its own
/// process while the tray icon and key listener start immediately.
pub fn show_if_needed() -> Result<(), Box<dyn Error>> {
    if State::has_seen_welcome() {
        return Ok(());
    }

    let child = Command::new(env::current_exe()?)
        .arg("--welcome")
        .stdout(Stdio::piped())
        .spawn()?;

    // Wait for the result on a side thread, so startup is not delayed.
    thread::spawn(move || {
        if let Ok(out) = child.wait_with_output() {
            if String::from_utf8_lossy(&out.stdout).trim() == "started" {
                State::mark_welcome_seen();
            }
        }
    });

    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoopBuilder::<String>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    // Created hidden, so the user never sees an empty white window.
    let window = WindowBuilder::new()
        .with_title("Welcome to Pico")
        .with_inner_size(LogicalSize::new(WIN_W, WIN_H))
        .with_resizable(false)
        .with_always_on_top(true)
        .with_visible(false)
        .build(&event_loop)?;

    if let Some(m) = event_loop.primary_monitor() {
        let scale = m.scale_factor();
        let size = m.size().to_logical::<f64>(scale);
        let pos = m.position().to_logical::<f64>(scale);

        window.set_outer_position(LogicalPosition::new(
            pos.x + (size.width - WIN_W) / 2.0,
            pos.y + (size.height - WIN_H) / 2.0 - 20.0,
        ));
    }

    let html = HTML.replace(
        "{{LOGO}}",
        &base64_encode(include_bytes!("../assets/icon.png")),
    );

    let webview = WebViewBuilder::new()
        .with_html(html)
        .with_ipc_handler(move |req| {
            let _ = proxy.send_event(req.body().to_string());
        })
        .build(&window)?;

    let start = Instant::now();
    let mut shown = false;

    event_loop.run(move |event, _, control_flow| {
        let _keep_alive = &webview;

        *control_flow = if shown {
            ControlFlow::Wait
        } else {
            ControlFlow::WaitUntil(start + SHOW_FALLBACK)
        };

        match event {
            Event::UserEvent(msg) => match msg.as_str() {
                "ready" if !shown => {
                    shown = true;
                    window.set_visible(true);
                    window.set_focus();
                }

                "started" => {
                    println!("started");
                    let _ = std::io::stdout().flush();
                    *control_flow = ControlFlow::Exit;
                }

                "close" => *control_flow = ControlFlow::Exit,

                _ => {}
            },

            // Fallback: the page did not report ready in time.
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) if !shown => {
                shown = true;
                window.set_visible(true);
                window.set_focus();
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            _ => {}
        }
    });
}

const HTML: &str = r##"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<style>
:root { color-scheme: light; --accent: #8493e8; --accent-hover: #6f80e0; }

* { box-sizing: border-box; }
html, body { height: 100%; }

body {
  margin: 0;
  background: #fff;
  color: #1d1d1f;
  font-family: system-ui, "Segoe UI", sans-serif;
  -webkit-font-smoothing: antialiased;
  user-select: none;
  cursor: default;
}

main {
  height: 100%;
  max-width: 880px;
  margin: 0 auto;
  padding: 36px 0 32px;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.logo {
  width: 44px;
  height: 44px;
  object-fit: contain;
  border-radius: 11px;
  margin-bottom: 22px;
}

.eyebrow {
  margin: 0 0 8px;
  color: #6e6e73;
  font-size: 15px;
  font-weight: 650;
}
.eyebrow-info {
  margin: 18px 0 0;
  color: #6e6e73;
  font-size: 13px;
  text-align: center;
}
h1 {
  margin: 0 0 34px;
  font-size: 40px;
  line-height: 1.1;
  letter-spacing: -1.6px;
  font-weight: 750;
  text-align: center;
}

.steps {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 18px;
  width: 100%;
  margin: 0;
  padding: 0;
  list-style: none;
}

.step {
  min-width: 0;
  padding: 22px 24px 26px;
  border-radius: 24px;
  background: #f4f4f6;
}

.art {
  height: 150px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  margin-bottom: 20px;
}

.key {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 66px;
  height: 66px;
  border-radius: 14px;
  border-bottom: 4px solid #0b0b0c;
  background: linear-gradient(#3d3d40, #1c1c1e);
  box-shadow: 0 14px 22px rgba(0, 0, 0, .16);
  color: #fff;
  font-size: 30px;
  font-weight: 400;
}

.key + .key { transform: translateY(5px); }

.key.ret {
  width: 132px;
  padding: 9px 13px;
  flex-direction: column;
  align-items: flex-end;
  justify-content: space-between;
  font-size: 13px;
}

.key.ret i { font-style: normal; font-size: 16px; }

.mini {
  width: 100%;
  max-width: 210px;
  padding: 9px;
  border-radius: 12px;
  background: #fff;
  box-shadow: 0 10px 26px rgba(0, 0, 0, .10);
}

.mini-q { margin-bottom: 7px; font-size: 10px; font-weight: 700; }

.mini-row { display: flex; gap: 4px; }

.mini-cell {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  padding: 9px 2px;
  border: 1px solid transparent;
  border-radius: 8px;
}

.mini-cell.on { background: #f1f1f3; border-color: #e4e4e8; }
.mini-cell i { font-style: normal; font-size: 22px; line-height: 1; }
.mini-cell b { color: #8a8a8e; font-size: 8px; font-weight: 500; white-space: nowrap; }

.n { margin: 0 0 6px; color: #6e6e73; font-size: 12px; font-weight: 650; }

h3 { margin: 0 0 8px; font-size: 22px; letter-spacing: -0.6px; font-weight: 700; }

.step p:last-child {
  margin: 0;
  color: #6e6e73;
  font-size: 13px;
  line-height: 1.65;
}

.step p b { color: #1d1d1f; font-weight: 650; }

.kb {
  display: inline-block;
  padding: 0 6px;
  border: 1px solid #d6d6da;
  border-bottom-width: 2px;
  border-radius: 5px;
  background: #fff;
  color: #333;
  font-size: 11px;
  line-height: 18px;
}

.spacer { flex: 1; min-height: 24px; }

button {
  border: 0;
  border-radius: 10px;
  padding: 13px 38px;
  background: rgb(0, 0, 0);
  color: #ffffff;
  font: 600 14px system-ui, "Segoe UI", sans-serif;
  cursor: pointer;
  transition: background .12s;
}
</style>
</head>
<body>
<main>
  <img class="logo" src="data:image/png;base64,{{LOGO}}" alt="Pico">

  <p class="eyebrow">How it works</p>
  <h1>Three keys. That is the whole app.</h1>

  <ol class="steps">
    <li class="step">
      <div class="art"><div class="key">/</div><div class="key">/</div></div>
      <p class="n">Step 1</p>
      <h3>Type two slashes.</h3>
      <p>Anywhere you can type. Pico takes the slashes back and opens the picker right at your text cursor. Or press <span class="kb">Right Alt</span> + <span class="kb">/</span></p>
    </li>

    <li class="step">
      <div class="art">
        <div class="mini">
          <div class="mini-q">/heart</div>
          <div class="mini-row">
            <span class="mini-cell on"><i>&#10084;&#65039;</i><b>/heart</b></span>
            <span class="mini-cell"><i>&#128525;</i><b>/heart_eyes</b></span>
            <span class="mini-cell"><i>&#128152;</i><b>/cupid</b></span>
          </div>
        </div>
      </div>
      <p class="n">Step 2</p>
      <h3>Pick.</h3>
      <p>Type a name like <b>heart</b>, <b>lol</b> or <b>thumbsup</b>, or move with the arrow keys. Your favorites are already on top.</p>
    </li>

    <li class="step">
      <div class="art"><div class="key ret"><i>&#8629;</i><span>enter</span></div></div>
      <p class="n">Step 3</p>
      <h3>Press Enter.</h3>
      <p>The emoji is in your text. The picker is gone. Keep typing. You never touched the mouse.</p>
    </li>
  </ol>
<p class="eyebrow-info">Reopen this guide anytime from the tray menu.</p> 
 <div class="spacer"></div>
  <button id="go">Get started</button>
</main>

<script>
  const send = m => window.ipc.postMessage(m);

  document.getElementById('go').onclick = () => send('started');

  window.addEventListener('keydown', e => {
    if (e.key === 'Enter') send('started');
    if (e.key === 'Escape') send('close');
  });

  send('ready');
</script>
</body>
</html>
"##;
