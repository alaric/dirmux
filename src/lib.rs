use async_trait::async_trait;
use std::path::PathBuf;
use anyhow::Result;
use tokio::sync::mpsc::UnboundedSender;

pub mod dirs;
pub mod exec;
pub mod options;
pub mod factory;
pub mod tag;
pub mod renderers;

pub use options::Options;

#[async_trait]
pub trait DirRunner: Send + Sync {
    async fn process(&self, dir: PathBuf, sender: UnboundedSender<CommandMessage>) -> Result<CommandOutput>;
}

pub trait Renderer {
    fn process(&self, msg: CommandMessage) -> Result<()>;
}

#[derive(Debug)]
pub enum CommandMessage {
    Increment(CommandOutput),
    Progress(CommandProgress),
    Final(Result<CommandOutput>),
}

#[derive(Debug)]
pub struct CommandProgress {
    progress: u32,
    out_of: u32,
    message: String,
    dir: PathBuf,
}

#[derive(Debug)]
pub struct CommandOutput {
    output: String,
    error: String,
    dir: PathBuf,
}

pub struct DebugRenderer {
}

impl Renderer for DebugRenderer {
    fn process(&self, msg: CommandMessage) -> Result<()> {
        dbg!(&msg);
        Ok(())
    }
}

/*
#[derive(Debug, Clone)]
pub struct Multiplex {
}

#[async_trait]
impl DirRunner for Multiplex {
    async fn process(&self, dir: PathBuf) -> Result<Output> {

        Ok(Output {
            dir,
            progress: 0,
            output: "".into(),
        })
    }
}
*/
