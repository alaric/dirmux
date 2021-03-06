use crate::CommandMessage;
use crate::CommandOutput;
use crate::DirRunner;
use anyhow::{bail, Result};
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::process::Command;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Clone)]
pub struct CommandRunner {
    pub cmd: Vec<String>,
}

#[async_trait]
impl DirRunner for CommandRunner {
    async fn process(
        &self,
        dir: PathBuf,
        _sender: UnboundedSender<CommandMessage>,
    ) -> Result<CommandOutput> {
        if let Some((progname, cmd)) = self.cmd.split_first() {
            let output = Command::new(progname).args(cmd).current_dir(&dir).output();
            let output = output.await?;

            Ok(CommandOutput {
                dir,
                output: String::from_utf8(output.stdout)?,
                error: String::from_utf8(output.stderr)?,
            })
        } else {
            bail!("No command provided for dir: {}", dir.display())
        }
    }
}
