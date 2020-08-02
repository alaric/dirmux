use crate::cmds::git::*;
use crate::exec::CommandRunner;
use crate::options::Options;
use crate::options::Subcommands;
use crate::renderers::*;
use crate::DirRunner;
use crate::Renderer;
use anyhow::bail;
use anyhow::Result;
use std::sync::Arc;

pub fn create_processors(opts: Options) -> Result<(Arc<dyn DirRunner>, Arc<dyn Renderer>)> {
    let processor: Arc<dyn DirRunner> = match &opts.cmd {
        Subcommands::RawCommand(cmd) => Arc::new(CommandRunner { cmd: cmd.to_vec() }),
        Subcommands::Exec(execcmd) => match &execcmd.cmd {
            crate::options::ExecCmd::RawCommand(cmd) => {
                Arc::new(CommandRunner { cmd: cmd.to_vec() })
            }
        },
        Subcommands::Status(opts) => Arc::new(StatusRunner { opts: opts.clone() }),
        Subcommands::Ffmerge(opts) => Arc::new(MergeRunner { opts: opts.clone() }),
        _ => bail!("Not a supported command type for directory running processing"),
    };

    let renderer: Arc<dyn Renderer> = match &opts.cmd {
        Subcommands::Status(_) => Arc::new(NullRender::default()),
        Subcommands::Ffmerge(_) => Arc::new(SimpleSectionRender::single_line()),
        _ => Arc::new(SimpleSectionRender::default()),
    };
    Ok((processor, renderer))
}
