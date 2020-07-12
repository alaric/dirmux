use crate::options::Options;
use crate::options::Subcommands;
use crate::renderers::*;
use crate::DirRunner;
use crate::Renderer;
use std::sync::Arc;
use anyhow::Result;
use anyhow::bail;

pub fn create_processors(opts: Options) -> Result<(Arc<dyn DirRunner>, Arc<dyn Renderer>)> {
    let cmd = match opts.cmd {
        Subcommands::RawCommand(cmd) => cmd,
        Subcommands::Exec(execcmd) => match execcmd.cmd {
            crate::options::ExecCmd::RawCommand(cmd) => cmd,
        },
        _ => bail!("Not a supported command type for directory running processing"),
    };

    let processor = crate::exec::CommandRunner { cmd };
    let renderer = SimpleSectionRender::default();

    Ok((Arc::new(processor), Arc::new(renderer)))
}
