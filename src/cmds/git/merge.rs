use crate::options::MergeOpts;
use crate::CommandMessage;
use crate::CommandOutput;
use crate::DirRunner;
use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task;

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
        let opts = self.opts.clone();
        let res = task::spawn_blocking(move || git_merge(opts, &dir)).await?;

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

fn git_merge(opts: MergeOpts, dir: &PathBuf) -> Result<String> {
    let repo = git2::Repository::open(dir)?;
    let head = repo.head()?;
    let (head_name, remote_ref) = match head.name() {
        Some(name) => (name, repo.branch_upstream_name(name)),
        None => return Ok(String::from("")),
    };

    let remote_ref = match remote_ref {
        Ok(remote_ref) => remote_ref,
        Err(_) => return Ok(String::from("")),
    };

    if let Some(remote_ref) = remote_ref.as_str() {
        let remote = repo.find_reference(remote_ref)?;
        let remote_annotated_commit = repo.reference_to_annotated_commit(&remote)?;
        let merge_analysis = repo.merge_analysis(&[&remote_annotated_commit])?;
        if merge_analysis.0.is_fast_forward() {
            let mut head_ref = repo.find_reference(head_name)?;
            let mut output = String::new();

            if opts.verbose {
                let head_tree = head_ref.peel_to_tree()?;
                let remote_tree = remote.peel_to_tree()?;
                let diff = repo.diff_tree_to_tree(Some(&head_tree), Some(&remote_tree), None)?;
                let diff_stats = diff.stats()?.to_buf(git2::DiffStatsFormat::FULL, 80);

                if let Some(diffoutput) = diff_stats?.as_str() {
                    output.push('\n');
                    output.push_str(diffoutput);
                }
            }

            let reflog_msg = format!(
                "Fast-Forward: Setting {} to id: {}",
                head_name,
                remote_annotated_commit.id()
            );

            if opts.dry == false {
                head_ref.set_target(remote_annotated_commit.id(), &reflog_msg)?;
                repo.set_head(&head_name)?;
                repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
            }

            output.push_str(&reflog_msg);
            output.push('\n');
            Ok(output)
        } else if merge_analysis.0.is_normal() {
            Ok(String::from("Cannot fast-forward\n"))
        } else if merge_analysis.0.is_up_to_date() {
            Ok(String::from(""))
        } else {
            Ok(String::from(""))
        }
    } else {
        return Ok(String::from(""));
    }
}
