use crate::CommandMessage;
use crate::Renderer;
use anyhow::Result;
use termion::{color, style};

#[derive(Default)]
pub struct SimpleSectionRender {}

impl Renderer for SimpleSectionRender {
    fn process(&self, msg: CommandMessage) -> Result<()> {
        match msg {
            CommandMessage::Final(Ok(msg)) => {
                if msg.output.len() > 0 {
                    println!("{}{}{}:{}", color::Fg(color::Rgb(200, 196, 0)), style::Bold, msg.dir.display(), style::Reset);
                    print!("{}", msg.output);
                }
                if msg.error.len() > 0 {
                    eprintln!("{}:", msg.dir.display());
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
