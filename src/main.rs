//! slashpick — type `//` anywhere, get a keyboard-only icon picker.
//!
//!   * `slashpick`            -> background listener (global key hook)
//!   * `slashpick --picker`   -> the modal (a tiny webview); prints the chosen icon to stdout
//!
//! The picker is HTML/CSS rendered by the OS webview, so emoji are drawn by the
//! system's own color emoji font (Apple Color Emoji on macOS, Segoe UI Emoji on
//! Windows, Noto Color Emoji on Linux) — exactly like 😍 in any other app.

use std::{
    error::Error,
    io::Write,
    process::Command,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::{Duration, Instant},
};

use enigo::{Direction, Enigo, Keyboard, Settings};
use rdev::{listen, EventType};
use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

mod emoji;
use emoji::EMOJIS;

/// True while the picker is open / text is being injected, so we ignore our own keystrokes.
static BUSY: AtomicBool = AtomicBool::new(false);

/// Max time between the two slashes.
const DOUBLE_TAP: Duration = Duration::from_millis(350);

// ───────────────────────────── main ─────────────────────────────

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::args().any(|a| a == "--picker") {
        return run_picker();
    }
    println!("slashpick running — type // anywhere. Ctrl+C to quit.");
    run_listener();
    Ok(())
}

// ─────────────────────── global key listener ───────────────────────

fn is_modifier(key: rdev::Key) -> bool {
    use rdev::Key::*;
    matches!(
        key,
        ShiftLeft | ShiftRight | ControlLeft | ControlRight | Alt | AltGr | MetaLeft | MetaRight | CapsLock
    )
}

fn run_listener() {
    // (time of first slash, was it directly after a ':' — i.e. "https://")
    let mut slash: Option<(Instant, bool)> = None;
    let mut prev: Option<String> = None;

    let result = listen(move |event| {
        if BUSY.load(Ordering::SeqCst) {
            slash = None;
            prev = None;
            return;
        }
        let EventType::KeyPress(key) = event.event_type else {
            return;
        };
        if is_modifier(key) {
            return; // don't let Shift etc. break "//" on layouts where / needs Shift
        }

        let ch = event.name.clone();
        let is_slash = match ch.as_deref() {
            Some(n) => n == "/",
            None => key == rdev::Key::Slash,
        };

        if is_slash {
            match slash.take() {
                Some((t, after_colon)) if t.elapsed() < DOUBLE_TAP => {
                    if !after_colon {
                        trigger();
                    }
                }
                _ => slash = Some((Instant::now(), prev.as_deref() == Some(":"))),
            }
        } else {
            slash = None;
        }
        prev = ch;
    });

    if let Err(e) = result {
        eprintln!("slashpick: could not start key listener: {e:?}");
        eprintln!("(macOS: grant Accessibility + Input Monitoring to your terminal/app)");
    }
}

fn trigger() {
    if BUSY.swap(true, Ordering::SeqCst) {
        return;
    }
    // Never block the keyboard hook: do the work on another thread.
    thread::spawn(|| {
        if let Err(e) = pick_and_insert() {
            eprintln!("slashpick: {e}");
        }
        BUSY.store(false, Ordering::SeqCst);
    });
}

fn pick_and_insert() -> Result<(), Box<dyn Error>> {
    let out = Command::new(std::env::current_exe()?).arg("--picker").output()?;
    let choice = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if choice.is_empty() {
        return Ok(()); // cancelled: leave the "//" untouched
    }

    // Give the OS a moment to hand focus back to the original window.
    thread::sleep(Duration::from_millis(200));

    let mut enigo = Enigo::new(&Settings::default())?;
    enigo.key(enigo::Key::Backspace, Direction::Click)?;
    enigo.key(enigo::Key::Backspace, Direction::Click)?;
    enigo.text(&choice)?;
    Ok(())
}

// ───────────────────────────── picker UI (webview) ─────────────────────────────

const WIN_W: f64 = 450.0;
const WIN_H: f64 = 380.0;

fn build_html() -> String {
    let data = EMOJIS
        .iter()
        .map(|i| format!("[\"{}\",\"{}\"]", i.glyph, i.name))
        .collect::<Vec<_>>()
        .join(",");
    HTML.replace("__DATA__", &format!("[{data}]"))
}

fn run_picker() -> Result<(), Box<dyn Error>> {
    let event_loop: EventLoop<String> = EventLoopBuilder::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title("slashpick")
        .with_inner_size(LogicalSize::new(WIN_W, WIN_H))
        .with_decorations(false)
        .with_transparent(true)
        .with_always_on_top(true)
        .with_resizable(false)
        .build(&event_loop)?;

    // Place it like Spotlight: horizontally centered, in the upper part of the screen.
    if let Some(m) = event_loop.primary_monitor() {
        let scale = m.scale_factor();
        let size = m.size().to_logical::<f64>(scale);
        let pos = m.position().to_logical::<f64>(scale);
        window.set_outer_position(LogicalPosition::new(
            pos.x + (size.width - WIN_W) / 2.0,
            pos.y + size.height * 0.22,
        ));
    }

    let builder = WebViewBuilder::new()
        .with_html(build_html())
        .with_transparent(true)
        .with_ipc_handler(move |req: wry::http::Request<String>| {
            let _ = proxy.send_event(req.body().clone());
        });

    #[cfg(any(target_os = "windows", target_os = "macos"))]
    let webview = builder.build(&window)?;
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().ok_or("no gtk vbox")?;
        builder.build_gtk(vbox)?
    };

    window.set_focus();

    let mut had_focus = false;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        let _keep_alive = &webview;

        match event {
            Event::UserEvent(msg) => {
                if let Some(glyph) = msg.strip_prefix("pick:") {
                    print!("{glyph}");
                    let _ = std::io::stdout().flush();
                }
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event: WindowEvent::Focused(focused), .. } => {
                if focused {
                    had_focus = true;
                } else if had_focus {
                    *control_flow = ControlFlow::Exit; // clicked away
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    })
}

const HTML: &str = r##"<!doctype html>
<html>
<head>
<meta charset="utf-8">
<style>
  :root {
    --bg: rgba(246,246,248,.98); --border: rgba(0,0,0,.11); --dim: #82828a;
    --text: #1d1d1f; --accent: 0,122,255; --hover: rgba(0,0,0,.05);
  }
  @media (prefers-color-scheme: dark) {
    :root {
      --bg: rgba(30,30,32,.98); --border: rgba(255,255,255,.10); --dim: #8c8c93;
      --text: #f5f5f7; --accent: 10,132,255; --hover: rgba(255,255,255,.07);
    }
  }
  * { box-sizing: border-box; }
  html, body { margin: 0; height: 100%; background: transparent; overflow: hidden;
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Segoe UI", system-ui, sans-serif;
    -webkit-font-smoothing: antialiased; user-select: none; cursor: default; }
  #panel { height: 100%; padding: 16px; display: flex; flex-direction: column;
    background: var(--bg); border: 1px solid var(--border); border-radius: 18px; color: var(--text); }
  #q { width: 100%; border: 0; outline: 0; background: transparent; color: var(--text);
    font: 400 20px/1.3 inherit; font-family: inherit; padding: 2px 2px 10px; user-select: text; }
  #q::placeholder { color: var(--dim); }
  .line { height: 1px; background: var(--border); margin: 0 0 8px; }
  #grid { flex: 1; overflow-y: auto; display: grid; grid-template-columns: repeat(8, 48px);
    grid-auto-rows: 48px; gap: 4px; align-content: start; scrollbar-width: none; }
  #grid::-webkit-scrollbar { display: none; }
  .cell { display: flex; align-items: center; justify-content: center; border-radius: 11px;
    font-size: 28px; line-height: 1;
    font-family: "Apple Color Emoji", "Segoe UI Emoji", "Noto Color Emoji", sans-serif; }
  .cell:hover { background: var(--hover); }
  .cell.sel { background: rgba(var(--accent), .30); }
  #foot { display: flex; justify-content: space-between; padding-top: 10px;
    font-size: 12px; color: var(--dim); }
  #name { font-size: 13px; }
</style>
</head>
<body>
<div id="panel">
  <input id="q" placeholder="Search emoji" autocomplete="off" spellcheck="false" autofocus>
  <div class="line"></div>
  <div id="grid"></div>
  <div id="foot"><span id="name"></span><span>↵ insert &nbsp;&nbsp; esc close</span></div>
</div>
<script>
  const EMOJIS = __DATA__;
  const COLS = 8;
  const q = document.getElementById('q');
  const grid = document.getElementById('grid');
  const nameEl = document.getElementById('name');
  let sel = 0, list = EMOJIS;

  const cap = w => w.charAt(0).toUpperCase() + w.slice(1);
  const send = m => window.ipc.postMessage(m);
  const pick = i => { if (list[i]) send('pick:' + list[i][0]); };

  function render() {
    const n = q.value.trim().toLowerCase();
    list = EMOJIS.filter(i => !n || i[1].includes(n));
    if (sel >= list.length) sel = Math.max(0, list.length - 1);
    grid.innerHTML = '';
    list.forEach((it, i) => {
      const d = document.createElement('div');
      d.className = 'cell' + (i === sel ? ' sel' : '');
      d.textContent = it[0];
      d.onclick = () => pick(i);
      grid.appendChild(d);
    });
    nameEl.textContent = list[sel] ? cap(list[sel][1].split(' ')[0]) : 'No results';
    const el = grid.children[sel];
    if (el) el.scrollIntoView({ block: 'nearest' });
  }

  q.addEventListener('input', () => { sel = 0; render(); });
  window.addEventListener('keydown', e => {
    let handled = true;
    switch (e.key) {
      case 'ArrowRight': sel = Math.min(sel + 1, list.length - 1); break;
      case 'ArrowLeft':  sel = Math.max(sel - 1, 0); break;
      case 'ArrowDown':  sel = Math.min(sel + COLS, list.length - 1); break;
      case 'ArrowUp':    sel = Math.max(sel - COLS, 0); break;
      case 'Enter':      pick(sel); break;
      case 'Escape':     send('cancel'); break;
      default: handled = false;
    }
    if (handled) { e.preventDefault(); render(); }
  });

  render();
  q.focus();
</script>
</body>
</html>"##;