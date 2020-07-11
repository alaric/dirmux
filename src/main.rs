use anyhow::Context;
use anyhow::Result;
use dirmux::options::Options;
use dirmux::CommandMessage;
use std::path::PathBuf;
use structopt::StructOpt;
use tokio::sync::mpsc::unbounded_channel;

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Options::from_args();
    let filename = match dirs::home_dir() {
        Some(homedir) => homedir.join(".dirmux.json"),
        None => PathBuf::from("/tmp/").join(".dirmux.json"),
    };
    let file = dirmux::dirs::read_file(&filename)
        .with_context(|| format!("Couldn't read config file: {}", filename.display()))?;

    // Short circuit tag command
    if let dirmux::options::Subcommands::Tag(tagopts) = &opts.cmd {
        return dirmux::tag::handle(&tagopts, &filename, &file);
    }

    let dirs = if let Some(t) = &opts.tag {
        dirmux::dirs::get_dirs(file, vec![&t])?
    } else {
        dirmux::dirs::get_dirs(file, vec![])?
    };

    let (processor, renderer) = dirmux::factory::create_processors(opts);
    let (tx, mut rx) = unbounded_channel();
    for dir in dirs {
        let tx = tx.clone();
        let processor = processor.clone();
        tokio::spawn(async move {
            let directory = dir.clone();
            let output = processor.process(dir, tx.clone()).await.with_context(|| {
                format!("Processing failed for directory: {}", directory.display())
            });
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
