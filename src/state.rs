
use std::{collections::HashMap, env, fs, path::PathBuf};

#[derive(Default)]
pub struct State {
    map: HashMap<String, (u32, u32)>, // glyph -> (use count, favorite order; 0 = not a favorite)
}

fn path() -> Option<PathBuf> {
    let base = if cfg!(target_os = "windows") {
        env::var_os("APPDATA").map(PathBuf::from)
    } else if cfg!(target_os = "macos") {
        env::var_os("HOME").map(|h| PathBuf::from(h).join("Library/Application Support"))
    } else {
        env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
    };
    base.map(|b| b.join("pico").join("state.tsv"))
}

impl State {
    pub fn load() -> Self {
        let mut map = HashMap::new();
        if let Some(text) = path().and_then(|p| fs::read_to_string(p).ok()) {
            for line in text.lines() {
                let mut it = line.split('\t');
                if let (Some(g), Some(c), Some(f)) = (it.next(), it.next(), it.next()) {
                    map.insert(g.to_string(), (c.parse().unwrap_or(0), f.parse().unwrap_or(0)));
                }
            }
        }
        State { map }
    }

    pub fn save(&self) {
        let Some(p) = path() else { return };
        if let Some(dir) = p.parent() {
            let _ = fs::create_dir_all(dir);
        }
        let mut out = String::new();
        for (g, (c, f)) in &self.map {
            out.push_str(&format!("{g}\t{c}\t{f}\n"));
        }
        let _ = fs::write(p, out);
    }

    /// Replace the whole favorites list (order matters: first = top).
    pub fn set_favs(&mut self, favs: &[&str]) {
        for v in self.map.values_mut() {
            v.1 = 0;
        }
        for (i, g) in favs.iter().enumerate() {
            self.map.entry(g.to_string()).or_insert((0, 0)).1 = (i + 1) as u32;
        }
    }

    pub fn record_use(&mut self, glyph: &str) {
        let e = self.map.entry(glyph.to_string()).or_insert((0, 0));
        e.0 += 1;
    }

    /// `{"🔥":[3,1], ...}` -> glyph: [use count, favorite order]
    pub fn to_json(&self) -> String {
        let items = self
            .map
            .iter()
            .map(|(g, (c, f))| format!("\"{}\":[{},{}]", g, c, f))
            .collect::<Vec<_>>()
            .join(",");
        format!("{{{items}}}")
    }
}