use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use termion::color::*;

lazy_static! {
    static ref STYLES: Arc<Mutex<HashMap<&'static str, Style>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub fn set_default_styles() {
    let mut map = STYLES.lock().unwrap();
    map.insert("git-modified", Style::new().fg(Yellow));
    map.insert("git-added", Style::new().fg(Green));
    map.insert("git-removed", Style::new().fg(Red));
    map.insert("git-unknown", Style::new().fg(Blue));
}

#[derive(Default, Clone)]
pub struct Style {
    fg: Option<String>,
    bg: Option<String>,
}

impl Style {
    pub fn id<T: AsRef<str>>(name: T) -> Self {
        let map = STYLES.lock().unwrap();
        map.get(name.as_ref()).unwrap_or(&Style::new()).clone()
    }

    pub fn new() -> Self {
        Style { fg: None, bg: None }
    }

    pub fn fg<T: Color>(mut self, col: T) -> Self {
        self.fg = Some(Fg(col).to_string());
        self
    }

    pub fn bg<T: Color>(mut self, col: T) -> Self {
        self.bg = Some(Bg(col).to_string());
        self
    }

    pub fn before(&self) -> String {
        let mut out = String::new();
        if let Some(fg) = &self.fg {
            out += fg;
        }
        if let Some(bg) = &self.bg {
            out += bg;
        }
        out
    }

    pub fn after(&self) -> String {
        let reset = termion::color::Reset;
        let mut out = reset.fg_str().to_string();
        out.push_str(reset.bg_str());
        out
    }

    pub fn output<T>(&self, input: T) -> String
    where
        T: AsRef<str> + std::fmt::Display,
    {
        format!("{}{}{}", self.before(), input, self.after())
    }
}
