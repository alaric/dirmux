use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, PartialEq, StructOpt)]
pub struct Options {
    #[structopt(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug, PartialEq, StructOpt)]
pub enum Subcommands {
    /// Manipulate the tagged directories
    Tag(TagOpts),
    Exec(ExecOpts),

    /// Any command is also allowed
    #[structopt(external_subcommand)]
    RawCommand(Vec<String>),
}

#[derive(Debug, PartialEq, StructOpt)]
pub enum TagSubcommands {
    Add(TagAddOpts),
    Remove(TagRemoveOpts),
    /// Garbage Collect non-existing directories
    ///
    /// This option will check the existence of all the directories and remove tags from
    /// non-existing directories.
    Gc
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct TagAddOpts {
    tag: String,
}

#[derive(Debug, PartialEq, StructOpt)]
pub struct TagRemoveOpts {
    tag: String,
    path: Option<PathBuf>,
}


#[derive(Debug, PartialEq, StructOpt)]
pub struct TagOpts {
    #[structopt(subcommand)]
    action: TagSubcommands,
}


#[derive(Debug, PartialEq, StructOpt)]
pub struct ExecOpts {
    cmd: Vector<String>
}
