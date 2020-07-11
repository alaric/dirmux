use crate::DirRunner;
use crate::Renderer;
use crate::Options;
use crate::options::Subcommands;
use crate::renderers::*;
use std::sync::Arc;

pub fn create_processors(opts: Options) -> (Arc<dyn DirRunner>, Arc<dyn Renderer>) {
    let cmd = match opts.cmd {
        Subcommands::RawCommand(cmd) => {
            cmd
        },
        Subcommands::Tag(tagcmd) => {
            vec!["ls".into(), "-l".into()] //TODO this makes no sense now whatsoever
        },
        Subcommands::Exec(execcmd) => {
            match execcmd.cmd {
                crate::options::ExecCmd::RawCommand(cmd) => cmd
            }
        }
    };

    let processor = crate::exec::CommandRunner { cmd };
    let renderer = SimpleSectionRender::default();

    (Arc::new(processor), Arc::new(renderer))
}
