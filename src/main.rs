use anyhow::Result;
use std::path::PathBuf;
use tokio::sync::mpsc::unbounded_channel;
use structopt::StructOpt;
use dirmux::CommandMessage;
use dirmux::Options;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Options::from_args();
    let filename = PathBuf::from("/Users/alaric/.grconfig.json"); 
    let file = dirmux::dirs::read_file(&filename)?;

    // Short circuit tag command
    if let dirmux::options::Subcommands::Tag(tagopts) = &opts.cmd {
        return dirmux::tag::handle(&tagopts, &filename, &file);
    }

    let (processor, renderer) = dirmux::factory::create_processors(opts);
    let dirs = dirmux::dirs::get_dirs(file, vec![])?;
    let (tx, mut rx) = unbounded_channel();
    for dir in dirs {
        let tx = tx.clone();
        let processor = processor.clone();
        tokio::spawn(async move {
            let output = processor.process(dir, tx.clone()).await;
            // TODO Handle error case by wrapping in dir variable
            tx.send(CommandMessage::Final(output)).unwrap();
        });
    }

    drop(tx);

    while let Some(msg) = rx.recv().await {
        renderer.process(msg)?;
    }

    Ok(())
}
