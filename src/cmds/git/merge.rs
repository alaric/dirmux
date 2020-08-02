use crate::options::MergeOpts;
use crate::CommandMessage;
use crate::DirRunner;
use crate::CommandOutput;
use anyhow::Result;
use async_trait::async_trait;
use crate::options::StatusOpts;
use crate::renderers::cleanup_path;
use crate::styling::Style;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task;
use std::path::PathBuf;

pub struct MergeRunner {
    pub opts: MergeOpts,
}

#[async_trait]
impl DirRunner for MergeRunner {
    async fn process(
        &self,
        dir: PathBuf,
        _sender: UnboundedSender<CommandMessage>,
    ) -> Result<CommandOutput> {
        let dir_out = dir.clone();
        let res = task::spawn_blocking(move || {
            git_merge(&dir)
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


fn git_merge(dir: &PathBuf) -> Result<String> {
    let repo = git2::Repository::open(dir)?;
    let head = repo.head()?;
    let (head_name, remote_ref) = match head.name() {
        Some(name) => {
            //dbg!(repo.branch_upstream_remote(name)?.as_str());
            (name, repo.branch_upstream_name(name))
        },
        None => return Ok(String::from("")),
    };

    let remote_ref = match remote_ref {
        Ok(remote_ref) => remote_ref,
        Err(_) => return Ok(String::from("")),
    };

    if let Some(remote_ref) = remote_ref.as_str() {
        let refr = repo.find_reference(remote_ref)?;
        let annotated_commit = repo.reference_to_annotated_commit(&refr)?;
        let merge_analysis = repo.merge_analysis(&[&annotated_commit])?;
        if merge_analysis.0.is_fast_forward() {
            dbg!(&head.name());
            dbg!(&refr.name());
            dbg!(&remote_ref);
            let head_annotated_commit = repo.reference_to_annotated_commit(&head)?;
            let mut head_ref = repo.find_reference(head_name)?;
            println!("Making {} from {} to {}", head_name, head_annotated_commit.id(), annotated_commit.id());
            let head_tree = head_ref.peel_to_tree()?;
            let remote_tree = refr.peel_to_tree()?;
            let diff = repo.diff_tree_to_tree(Some(&head_tree), Some(&remote_tree), None)?;
            let diff_stats = diff.stats()?.to_buf(git2::DiffStatsFormat::FULL, 80);
            dbg!(&diff_stats?.as_str());
            let reflog_msg = format!("Fast-Forward: Setting {} to id: {}", head_name, annotated_commit.id());
            head_ref.set_target(annotated_commit.id(), &reflog_msg)?;
            repo.set_head(&head_name)?;
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
            
            Ok(reflog_msg)
        }
        else if merge_analysis.0.is_normal() {
            Ok(String::from("Cannot fast-forward\n"))
        }
        else if merge_analysis.0.is_up_to_date() {
            Ok(String::from(""))
        }
        else {
            Ok(String::from(""))
        }
    }
    else {
        return Ok(String::from(""));
    }
}
