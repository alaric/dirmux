use anyhow::Result;
use std::path::PathBuf;
use tokio::sync::mpsc::unbounded_channel;
use structopt::StructOpt;
use dirmux::DirRunner;
use dirmux::CommandMessage;
use dirmux::Options;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = dirmux::Options::from_args();
    dbg!(&opts);
    let file = dirmux::dirs::read_file(&PathBuf::from("/Users/alaric/.grconfig.json"))?;
    let dirs = dirmux::dirs::get_dirs(file, vec![])?;

    let cmd = match opts.cmd {
        dirmux::options::Subcommands::RawCommand(cmd) => {
            cmd
        },
        dirmux::options::Subcommands::Tag(tagcmd) => {
            vec!["ls".into(), "-l".into()]
        }
    };
    
    let processor = dirmux::exec::CommandRunner { cmd };
    let (tx, mut rx) = unbounded_channel();
    for dir in dirs {
        let tx = tx.clone();
        let processor = processor.clone();
        tokio::spawn(async move {
            let output = processor.process(dir, tx.clone()).await;
            tx.send(CommandMessage::Final(output)).unwrap();
        });
    }

    drop(tx);

    while let Some(res) = rx.recv().await {
        println!("got = {:?}", res);
    }

    Ok(())
}
