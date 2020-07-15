use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub struct Options {
    #[structopt(short)]
    pub tag: Option<String>,

    #[structopt(short,long, default_value="10")]
    pub jobs: usize,

    #[structopt(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub enum Subcommands {
    Status(StatusOpts),
    /// Manipulate the tagged directories
    Tag(TagOpts),
    Exec(ExecOpts),

    /// Any command is also allowed
    #[structopt(external_subcommand)]
    RawCommand(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub enum TagSubcommands {
    Add(TagAddOpts),
    Remove(TagRemoveOpts),
    /// Garbage Collect non-existing directories
    ///
    /// This option will check the existence of all the directories and remove tags from
    /// non-existing directories.
    Gc,
}

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub struct StatusOpts {
}

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub struct TagAddOpts {
    pub tag: String,
    pub path: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub struct TagRemoveOpts {
    pub tag: String,
    pub path: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub struct TagOpts {
    #[structopt(subcommand)]
    pub action: TagSubcommands,
}

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub struct ExecOpts {
    #[structopt(subcommand)]
    pub cmd: ExecCmd,
}

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub enum ExecCmd {
    #[structopt(external_subcommand)]
    RawCommand(Vec<String>),
}
