use crate::CommandMessage;
use crate::Renderer;
use anyhow::Result;

#[derive(Default)]
pub struct SimpleSectionRender {}

impl Renderer for SimpleSectionRender {
    fn process(&self, msg: CommandMessage) -> Result<()> {
        match msg {
            CommandMessage::Final(Ok(msg)) => {
                if msg.output.len() > 0 {
                    println!("{}:", msg.dir.display());
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
