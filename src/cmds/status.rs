use anyhow::Result;
use async_trait::async_trait;
use crate::CommandMessage;
use crate::DirRunner;
use crate::CommandOutput;
use crate::options::StatusOpts;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task;
use std::path::PathBuf;

pub struct StatusRunner {
    pub opts: StatusOpts,
}

#[async_trait]
impl DirRunner for StatusRunner {
    async fn process(
        &self,
        dir: PathBuf,
        _sender: UnboundedSender<CommandMessage>,
    ) -> Result<CommandOutput> {
        let dir_out = dir.clone();
        let res = task::spawn_blocking(move || {
            git_status(&dir)
        }).await?;

        match res {
            Ok(s) => Ok(CommandOutput {
            dir: dir_out,
            output: s,
            error: String::from(""),
        }),
        Err(e) => Ok(CommandOutput {
            dir: dir_out,
            error: e.to_string(),
            output: String::from(""),
        }),
        }
    }
}

fn git_status(dir: &PathBuf) -> Result<String> {

    let mut repo = git2::Repository::open(dir)?;
    let mut status_options = git2::StatusOptions::new();
    let statuses = repo.statuses(Some(&mut status_options))?;
    statuses.iter().
    for i in statuses.iter() {
        dbg!(i.status());
        dbg!(i.path());
    }
    Ok(String::from("test status 2\n"))
}
