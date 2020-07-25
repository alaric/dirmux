use crate::CommandMessage;
use crate::Renderer;
use anyhow::Result;
use std::path::PathBuf;
use termion::{color, style};

#[derive(Default)]
pub struct NullRender {}

impl Renderer for NullRender {
    fn process(&self, msg: CommandMessage) -> Result<()> {
        match msg {
            CommandMessage::Final(Ok(msg)) => print!("{}", msg.output),
            CommandMessage::Final(Err(msg)) => eprint!("{}", msg),
            _ => {}
        };
        Ok(())
    }
}

#[derive(Default)]
pub struct SimpleSectionRender {
    single_line: bool,
}

impl Renderer for SimpleSectionRender {
    fn process(&self, msg: CommandMessage) -> Result<()> {
        match msg {
            CommandMessage::Final(Ok(msg)) => {
                let newline = if self.single_line { " " } else { "\n" };
                if msg.output.len() > 0 {
                    print!(
                        "{}{}{}:{}{}",
                        color::Fg(color::Rgb(200, 196, 0)),
                        style::Bold,
                        cleanup_path(&msg.dir)?,
                        style::Reset,
                        newline,
                    );
                    print!("{}", msg.output);
                }
                if msg.error.len() > 0 {
                    eprint!("{}:{}", msg.dir.display(), newline);
                    eprint!("{}", msg.error);
                }
            }
            CommandMessage::Final(Err(msg)) => {
                eprintln!("Err: {}", msg);
            }
            _ => {}
        }
        Ok(())
    }
}

impl SimpleSectionRender {
    pub fn single_line() -> Self {
        SimpleSectionRender { single_line: true }
    }
}

pub fn cleanup_path(path: &PathBuf) -> Result<String> {
    let res = match dirs::home_dir() {
        Some(homedir) => {
            if path.starts_with(&homedir) {
                let mut dir = path.strip_prefix(&homedir)?.to_string_lossy().to_string();
                dir.insert_str(0, "~/");
                dir
            } else {
                path.to_string_lossy().to_string()
            }
        }
        None => path.to_string_lossy().to_string(),
    };

    Ok(res)
}
