use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::sync::mpsc::UnboundedSender;

/// File handling and production of directories to work with
pub mod dirs;
/// Command-line execution runners
pub mod exec;
/// Creating different behaviours from the program options
pub mod factory;
/// Program and command line options
pub mod options;
/// Render the directory's outputs as text
pub mod renderers;
/// Managing the tags
pub mod tag;
/// Styling for commands
pub mod styling;

pub mod cmds;

#[async_trait]
pub trait DirRunner: Send + Sync {
    async fn process(
        &self,
        dir: PathBuf,
        sender: UnboundedSender<CommandMessage>,
    ) -> Result<CommandOutput>;
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

pub struct DebugRenderer {}

impl Renderer for DebugRenderer {
    fn process(&self, msg: CommandMessage) -> Result<()> {
        dbg!(&msg);
        Ok(())
    }
}

