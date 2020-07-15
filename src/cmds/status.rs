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

    let repo = git2::Repository::open(dir)?;
    let head = repo.head()?;
    let shorthand = head.shorthand().or(Some(""));
    let mut status_options = git2::StatusOptions::new();
    status_options.include_untracked(true);
    let statuses = repo.statuses(Some(&mut status_options))?;
    let mut modified_count = 0;   // M
    let mut added_count = 0;      // A
    let mut deleted_count = 0;    // D
    let mut renamed_count = 0;    // R
    let mut typechange_count = 0; // T
    let mut ignored_count = 0;    // !
    let mut conflicted_count = 0; // C
    let mut unknown_count = 0;    // ?
    for i in statuses.iter() {
        let s = i.status();
        if s.is_index_new() {
            added_count += 1;
        }
        if s.is_wt_new() {
            unknown_count += 1;
        }
        if s.is_index_modified() || s.is_wt_modified() {
            modified_count += 1;
        }
        if s.is_index_deleted() || s.is_wt_deleted() {
            deleted_count += 1;
        }
        if s.is_index_renamed() || s.is_wt_renamed() {
            renamed_count += 1;
        }
        if s.is_index_typechange() || s.is_wt_typechange() {
            typechange_count += 1;
        }
        if s.is_ignored() {
            ignored_count += 1;
        }
        if s.is_conflicted() {
            conflicted_count += 1;
        }
        //dbg!(i.status());
        //dbg!(i.path());
    }

    let mut output: Vec<String> = vec![];
    status_fmt(&mut output, "M", modified_count);
    status_fmt(&mut output, "A", added_count);
    status_fmt(&mut output, "D", deleted_count);
    status_fmt(&mut output, "R", renamed_count);
    status_fmt(&mut output, "T", typechange_count);
    status_fmt(&mut output, "!", ignored_count);
    status_fmt(&mut output, "C", conflicted_count);
    status_fmt(&mut output, "?", unknown_count);
    let statuses = output.join(" ");
    let mut output = String::from("");
    if statuses.is_empty() && shorthand == Some("master") {
        Ok(output)
    }
    else {
        output.push_str(format!("{:12}", statuses).as_ref());
        if let Some(s) = shorthand {
            output.push_str(format!(" {:12}", s).as_ref())
        }
        output.push('\n');
        Ok(output)
    }
}

fn status_fmt(output: &mut Vec<String>, suff: &str, count: u32) {
    if count > 0 {
        output.push(format!("{}{}", count, suff));
    }
}
