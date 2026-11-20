use std::{
    env,
    error::Error,
    io::Write,
    process::Command,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::{Duration, Instant},
};

use enigo::{Direction, Enigo, Keyboard, Settings};
use rdev::{listen, EventType, Key as K};
use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

mod emoji;
mod state;
use emoji::EMOJIS;
use state::State;

/// True while the picker is open / text is being injected, so we ignore our own keystrokes.
static BUSY: AtomicBool = AtomicBool::new(false);

/// Max time between the characters of the trigger.
const DOUBLE_TAP: Duration = Duration::from_millis(400);

// ───────────────────────────── main ─────────────────────────────

fn arg_value(flag: &str) -> Option<String> {
    let args: Vec<String> = env::args().collect();
    args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1).cloned())
}

fn main() -> Result<(), Box<dyn Error>> {
    if env::args().any(|a| a == "--picker") {
        let prefix = arg_value("--prefix").unwrap_or_else(|| "/".to_string());
        return run_picker(&prefix);
    }

    let trigger = arg_value("--trigger").unwrap_or_else(|| "//".to_string());
    let chars: Vec<String> = trigger.chars().map(|c| c.to_string()).collect();
    if chars.is_empty() || chars.len() > 2 {
        eprintln!("--trigger must be 1 or 2 characters, e.g. --trigger \";;\"");
        std::process::exit(2);
    }

    println!("pico running — type {trigger} in any app, or press Ctrl+Alt+E. Ctrl+C to quit.");
    run_listener(chars);
    Ok(())
}

// ─────────────────────── global key listener ───────────────────────

/// `//` only opens at the start of typing, after a space, or after an emoji/symbol —
/// never inside a word or link (`and//or`, `https://`).
fn starts_ok(prev: Option<&str>) -> bool {
    match prev.and_then(|p| p.chars().next()) {
        None => true,
        Some(c) => c.is_whitespace() || !(c.is_alphanumeric() || c == ':' || c == '.'),
    }
}

fn run_listener(trigger: Vec<String>) {
    let prefix: String = trigger.concat();
    let n = trigger.len();

    let (mut ctrl, mut alt, mut meta) = (false, false, false);
    let mut progress = 0usize; // how many trigger characters we've seen so far
    let mut started = Instant::now();
    let mut first_ok = true;
    let mut prev: Option<String> = None;

    let result = listen(move |event| {
        // Track modifiers first, even while busy, so they never get stuck.
        match event.event_type {
            EventType::KeyPress(k) => match k {
                K::ControlLeft | K::ControlRight => ctrl = true,
                K::Alt | K::AltGr => alt = true,
                K::MetaLeft | K::MetaRight => meta = true,
                _ => {}
            },
            EventType::KeyRelease(k) => match k {
                K::ControlLeft | K::ControlRight => ctrl = false,
                K::Alt | K::AltGr => alt = false,
                K::MetaLeft | K::MetaRight => meta = false,
                _ => {}
            },
            _ => {}
        }

        if BUSY.load(Ordering::SeqCst) {
            progress = 0;
            prev = None;
            return;
        }
        let EventType::KeyPress(key) = event.event_type else {
            return;
        };
        if matches!(
            key,
            K::ControlLeft
                | K::ControlRight
                | K::Alt
                | K::AltGr
                | K::MetaLeft
                | K::MetaRight
                | K::ShiftLeft
                | K::ShiftRight
                | K::CapsLock
        ) {
            return; // modifiers alone never break a trigger
        }

        // Keyboard shortcut: Ctrl+Alt+E (nothing was typed, so nothing to erase).
        if ctrl && alt && key == K::KeyE {
            progress = 0;
            fire(0, "/".to_string());
            return;
        }
        // Ctrl/Cmd/Alt combos (e.g. Ctrl+/ = "toggle comment") are never the trigger.
        if ctrl || alt || meta {
            progress = 0;
            prev = None;
            return;
        }

        let ch = event.name.clone();
        let typed = ch.as_deref();

        if progress > 0
            && (started.elapsed() > DOUBLE_TAP || typed != Some(trigger[progress].as_str()))
        {
            progress = 0;
        }
        if typed == Some(trigger[progress].as_str()) {
            if progress == 0 {
                first_ok = starts_ok(prev.as_deref());
                started = Instant::now();
            }
            progress += 1;
            if progress == n {
                progress = 0;
                if first_ok {
                    fire(n, prefix.clone());
                }
            }
        }
        prev = ch;
    });

    if let Err(e) = result {
        eprintln!("pico: could not start key listener: {e:?}");
        eprintln!("(macOS: grant Accessibility + Input Monitoring to your terminal/app)");
    }
}

/// Open the picker. `erase` = how many typed trigger characters to remove afterwards.
fn fire(erase: usize, prefix: String) {
    if BUSY.swap(true, Ordering::SeqCst) {
        return;
    }
    // Never block the keyboard hook: do the work on another thread.
    thread::spawn(move || {
        if let Err(e) = pick_and_insert(erase, &prefix) {
            eprintln!("pico: {e}");
        }
        thread::sleep(Duration::from_millis(150));
        BUSY.store(false, Ordering::SeqCst);
    });
}

fn pick_and_insert(erase: usize, prefix: &str) -> Result<(), Box<dyn Error>> {
    let out = Command::new(env::current_exe()?)
        .arg("--picker")
        .arg("--prefix")
        .arg(prefix)
        .output()?;
    let reply = String::from_utf8_lossy(&out.stdout).trim().to_string();

    // "pick:<emoji>" -> insert, "cancel" (Esc) -> just take the trigger back,
    // anything else (clicked away) -> leave the text field alone.
    let glyph = reply.strip_prefix("pick:");
    if glyph.is_none() && reply != "cancel" {
        return Ok(());
    }

    // Give the OS a moment to hand focus back to the original window.
    thread::sleep(Duration::from_millis(200));

    let mut enigo = Enigo::new(&Settings::default())?;
    for _ in 0..erase {
        enigo.key(enigo::Key::Backspace, Direction::Click)?;
    }
    if let Some(g) = glyph {
        enigo.text(g)?;
    }
    Ok(())
}

// ───────────────────────────── picker UI (webview) ─────────────────────────────

const WIN_W: f64 = 280.0;
const WIN_H: f64 = 360.0;

fn js_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

fn build_html(prefix: &str, state: &State) -> String {
    let data = EMOJIS
        .iter()
        .map(|e| format!("[\"{}\",\"{}\",\"{}\"]", e.glyph, e.name, e.cat))
        .collect::<Vec<_>>()
        .join(",");
    HTML.replace("__DATA__", &format!("[{data}]"))
        .replace("__STATE__", &state.to_json())
        .replace("__PREFIX__", &js_str(prefix))
}

fn run_picker(prefix: &str) -> Result<(), Box<dyn Error>> {
    let event_loop: EventLoop<String> = EventLoopBuilder::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title("pico")
        .with_inner_size(LogicalSize::new(WIN_W, WIN_H))
        .with_decorations(false)
        .with_transparent(true)
        .with_always_on_top(true)
        .with_resizable(false)
        .build(&event_loop)?;

    // Horizontally centered, in the upper part of the screen.
    if let Some(m) = event_loop.primary_monitor() {
        let scale = m.scale_factor();
        let size = m.size().to_logical::<f64>(scale);
        let pos = m.position().to_logical::<f64>(scale);
        window.set_outer_position(LogicalPosition::new(
            pos.x + (size.width - WIN_W) / 2.0,
            pos.y + size.height * 0.22,
        ));
    }

    let mut state = State::load();

    let builder = WebViewBuilder::new()
        .with_html(build_html(prefix, &state))
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
                if let Some(list) = msg.strip_prefix("favs:") {
                    let favs: Vec<&str> = list.split('|').filter(|s| !s.is_empty()).collect();
                    state.set_favs(&favs);
                    state.save();
                } else if let Some(g) = msg.strip_prefix("pick:") {
                    state.record_use(g);
                    state.save();
                    print!("pick:{g}");
                    let _ = std::io::stdout().flush();
                    *control_flow = ControlFlow::Exit;
                } else if msg == "cancel" {
                    print!("cancel");
                    let _ = std::io::stdout().flush();
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::WindowEvent { event: WindowEvent::Focused(focused), .. } => {
                if focused {
                    had_focus = true;
                } else if had_focus {
                    *control_flow = ControlFlow::Exit; // clicked away: touch nothing
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
    --bg: rgba(255,255,255,.99); --fg: #111113; --dim: #8e8e93; --line: rgba(0,0,0,.08);
    --sel: rgba(0,0,0,.065); --hover: rgba(0,0,0,.035); --ci-on: rgba(0,0,0,.08);
  }
  :root { color-scheme: light; }
  * { box-sizing: border-box; }
  html, body { margin: 0; height: 100%; background: transparent; overflow: hidden;
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "Segoe UI Variable Text", "Segoe UI", system-ui, sans-serif;
    -webkit-font-smoothing: antialiased; user-select: none; cursor: default; }
  #panel { height: 100%; padding: 12px 12px 0; display: flex; flex-direction: column;
    background: var(--bg); border: 1px solid var(--line); border-radius: 14px; color: var(--fg); }

  /* header: category title + hint, or the text you are typing */
  #head { position: relative; height: 24px; display: flex; align-items: center; justify-content: space-between;
    padding: 0 4px; margin-bottom: 6px; }
  #title { font-size: 11px; font-weight: 600; }
  #hint { font-size: 10px; color: var(--dim); }
  #typed { display: none; align-items: center; font-size: 15px; font-weight: 600; letter-spacing: -.1px; }
  #typed .p { color: var(--dim); font-weight: 400; margin-right: 1px; }
  .caret { width: 1.5px; height: 16px; background: var(--fg); margin-left: 1px; animation: blink 1s steps(1) infinite; }
  @keyframes blink { 50% { opacity: 0; } }
  #q { position: absolute; inset: 0; width: 100%; opacity: 0; border: 0; outline: 0; background: transparent; }

  /* emoji grid: 3 columns, big glyph, /name underneath, star top-right */
  #body { flex: 1; overflow-y: auto; scrollbar-width: none; padding-bottom: 8px; }
  #body::-webkit-scrollbar { display: none; }
  .grid { display: grid; grid-template-columns: repeat(3, 1fr); grid-auto-rows: 84px; gap: 6px; }
  .cell { position: relative; display: flex; flex-direction: column; align-items: center; justify-content: center;
    border-radius: 12px; }
  .cell:hover { background: var(--hover); }
  .cell.sel { background: var(--sel); }
  .glyph { font-size: 34px; line-height: 1.1;
    font-family: "Apple Color Emoji", "Segoe UI Emoji", "Noto Color Emoji", sans-serif; }
  .lbl { margin-top: 5px; font-size: 11px; color: var(--fg); max-width: 100%; padding: 0 6px;
    white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .star { position: absolute; top: 6px; right: 9px; font-size: 12px; line-height: 1; color: var(--dim); opacity: .6; }
  .star.on { color: var(--fg); opacity: .75; }
  .empty { color: var(--dim); font-size: 12px; padding: 16px 6px; }

  /* category bar */
  #cats { display: flex; justify-content: space-between; align-items: center; height: 40px;
    margin: 0 -12px; padding: 0 10px; border-top: 1px solid var(--line); }
  .ci { width: 24px; height: 24px; display: flex; align-items: center; justify-content: center;
    border-radius: 7px; font-size: 14px; line-height: 1; filter: grayscale(1); opacity: .6; }
  .ci:hover { background: var(--hover); opacity: .9; }
  .ci.on { background: var(--ci-on); opacity: 1; }

  /* footer: three key hints */
  #foot { display: grid; grid-template-columns: 1fr 1fr 1fr; height: 34px; align-items: center;
    margin: 0 -12px; border-top: 1px solid var(--line); font-size: 10.5px; color: var(--dim); text-align: center; }
  #foot > div { border-left: 1px solid var(--line); line-height: 14px; }
  #foot > div:first-child { border-left: 0; }
  #foot b { color: var(--fg); font-weight: 600; margin-right: 3px; }
</style>
</head>
<body>
<div id="panel">
  <div id="head">
    <span id="title"></span><span id="hint">Shift + ← → to reorder</span>
    <span id="typed"><span class="p" id="pfx"></span><span id="typedtext"></span><span class="caret"></span></span>
    <input id="q" autocomplete="off" spellcheck="false" autofocus>
  </div>
  <div id="body"></div>
  <div id="cats"></div>
  <div id="foot">
    <div><b>Enter</b>Insert</div>
    <div><b>Space</b><span id="favlabel">Favorite</span></div>
    <div><b>Tab</b>Categories</div>
  </div>
</div>
<script>
  const DATA = __DATA__;      // [glyph, keywords, category]
  const STATE = __STATE__;    // glyph -> [times used, favorite order (0 = no)]
  const PREFIX = __PREFIX__;
  const COLS = 3;
  const CATS = ['Favorites', ...new Set(DATA.map(d => d[2]))];
  const ICON = { Favorites: '☆', Status: '✅', Faces: '😀', Hands: '👋', Animals: '🐻', Nature: '🌿',
                 Food: '🍎', Activities: '⚽', Travel: '🚗', Objects: '💡', Symbols: '🔣' };

  const $ = id => document.getElementById(id);
  const qEl = $('q'), catsEl = $('cats'), bodyEl = $('body');
  const mk = (tag, cls, text) => { const e = document.createElement(tag); e.className = cls; if (text != null) e.textContent = text; return e; };
  const send = m => window.ipc.postMessage(m);
  const cnt = g => (STATE[g] || [0, 0])[0];
  const uniq = arr => { const s = new Set(); return arr.filter(d => s.has(d[0]) ? false : (s.add(d[0]), true)); };
  const byGlyph = {}; DATA.forEach(d => { if (!(d[0] in byGlyph)) byGlyph[d[0]] = d; });

  // ordered favorites (first = top)
  let FAVS = Object.keys(STATE).filter(g => STATE[g][1] > 0).sort((a, b) => STATE[a][1] - STATE[b][1]);
  const isFav = g => FAVS.includes(g);
  const favRank = g => { const i = FAVS.indexOf(g); return i < 0 ? 1e9 : i; };
  const saveFavs = () => send('favs:' + FAVS.join('|'));

  let items = [], sel = 0, selGlyph = null, catIdx = FAVS.length ? 0 : 1;
  $('pfx').textContent = PREFIX;

  function compute() {
    const q = qEl.value.trim().toLowerCase().replace(/^\/+/, '');
    if (q) {
      const out = [];
      DATA.forEach((d, i) => {
        let best = 9;
        for (const t of d[1].split(' ')) {
          const s = t === q ? 0 : t.startsWith(q) ? 1 : t.includes(q) ? 2 : 9;
          if (s < best) best = s;
        }
        if (best < 9) out.push({ d, i, s: best });
      });
      // exact names first, then prefix matches; favorites and most-used on top
      out.sort((a, b) => a.s - b.s || favRank(a.d[0]) - favRank(b.d[0]) || cnt(b.d[0]) - cnt(a.d[0]) || a.i - b.i);
      items = uniq(out.map(o => o.d));
    } else if (CATS[catIdx] === 'Favorites') {
      // favorites first (in your order), then everything else by how often you use it
      const favs = FAVS.map(g => byGlyph[g]).filter(Boolean);
      const rest = uniq(DATA.map((d, i) => ({ d, i })).filter(o => !isFav(o.d[0]))
        .sort((a, b) => cnt(b.d[0]) - cnt(a.d[0]) || a.i - b.i).map(o => o.d));
      items = [...favs, ...rest];
    } else {
      items = uniq(DATA.map((d, i) => ({ d, i })).filter(o => o.d[2] === CATS[catIdx])
        .sort((a, b) => favRank(a.d[0]) - favRank(b.d[0]) || cnt(b.d[0]) - cnt(a.d[0]) || a.i - b.i).map(o => o.d));
    }
  }

  function pick(i) { if (items[i]) send('pick:' + items[i][0]); }
  function toggleFav(i) {
    const g = items[i][0];
    FAVS = isFav(g) ? FAVS.filter(x => x !== g) : [...FAVS, g];
    selGlyph = g;
    saveFavs();
  }
  function reorder(dir) {
    const g = items[sel] && items[sel][0];
    const i = FAVS.indexOf(g), j = i + dir;
    if (i < 0 || j < 0 || j >= FAVS.length) return;
    [FAVS[i], FAVS[j]] = [FAVS[j], FAVS[i]];
    selGlyph = g;
    saveFavs();
  }

  function render() {
    compute();
    if (selGlyph) { const k = items.findIndex(d => d[0] === selGlyph); if (k >= 0) sel = k; selGlyph = null; }
    sel = Math.max(0, Math.min(sel, items.length - 1));
    const typing = qEl.value.length > 0;

    // header
    $('title').textContent = CATS[catIdx] === 'Favorites' ? 'Favorites' : CATS[catIdx];
    $('title').style.display = typing ? 'none' : '';
    $('hint').style.display = (!typing && CATS[catIdx] === 'Favorites' && FAVS.length > 1) ? '' : 'none';
    $('typed').style.display = typing ? 'flex' : 'none';
    $('typedtext').textContent = qEl.value;

    // grid
    bodyEl.innerHTML = '';
    if (!items.length) {
      bodyEl.appendChild(mk('div', 'empty', 'No emoji found.'));
    } else {
      const grid = mk('div', 'grid');
      items.forEach((d, i) => {
        const fav = isFav(d[0]);
        const c = mk('div', 'cell' + (i === sel ? ' sel' : ''));
        c.append(mk('div', 'glyph', d[0]), mk('div', 'lbl', '/' + d[1].split(' ')[0]));
        const s = mk('div', 'star' + (fav ? ' on' : ''), fav ? '★' : '☆');
        s.onclick = e => { e.stopPropagation(); toggleFav(i); render(); };
        c.append(s);
        c.onclick = () => pick(i);
        grid.appendChild(c);
      });
      bodyEl.appendChild(grid);
    }
    const s = bodyEl.querySelector('.sel'); if (s) s.scrollIntoView({ block: 'nearest' });

    // category bar
    catsEl.innerHTML = '';
    CATS.forEach((c, i) => {
      const b = mk('div', 'ci' + (!typing && i === catIdx ? ' on' : ''), ICON[c] || '•');
      b.title = c;
      b.onclick = () => { catIdx = i; sel = 0; qEl.value = ''; render(); };
      catsEl.appendChild(b);
    });

    $('favlabel').textContent = items[sel] && isFav(items[sel][0]) ? 'Unfavorite' : 'Favorite';
  }

  qEl.addEventListener('input', () => { sel = 0; render(); });
  document.addEventListener('mousedown', e => { e.preventDefault(); qEl.focus(); }); // keep typing focus
  window.addEventListener('keydown', e => {
    const n = items.length, typing = qEl.value.length > 0;
    let handled = true;
    switch (e.key) {
      case 'ArrowLeft':  if (e.shiftKey && !typing && CATS[catIdx] === 'Favorites') reorder(-1); else sel -= 1; break;
      case 'ArrowRight': if (e.shiftKey && !typing && CATS[catIdx] === 'Favorites') reorder(1); else sel += 1; break;
      case 'ArrowUp':    sel -= COLS; break;
      case 'ArrowDown':  sel += COLS; break;
      case 'Tab':        if (!typing) { catIdx = (catIdx + (e.shiftKey ? CATS.length - 1 : 1)) % CATS.length; sel = 0; } break;
      case 'Enter':      pick(sel); break;
      case 'Escape':     send('cancel'); break;
      case 'Backspace':  if (!typing) send('cancel'); else handled = false; break;
      case ' ':          if (n) toggleFav(sel); break; // works while searching too (names have no spaces)
      default: handled = false;
    }
    if (handled) { e.preventDefault(); render(); }
  });

  render();
  qEl.focus();
</script>
</body>
</html>"##;