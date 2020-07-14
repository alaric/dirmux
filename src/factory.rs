use crate::cmds::*;
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
    let processor: Arc<dyn DirRunner> = match opts.cmd {
        Subcommands::RawCommand(cmd) => Arc::new(CommandRunner { cmd }),
        Subcommands::Exec(execcmd) => match execcmd.cmd {
            crate::options::ExecCmd::RawCommand(cmd) => Arc::new(CommandRunner { cmd }),
        },
        Subcommands::Status(opts) => Arc::new(StatusRunner { opts }),
        _ => bail!("Not a supported command type for directory running processing"),
    };

    let renderer = SimpleSectionRender::default();
    Ok((processor, Arc::new(renderer)))
}
