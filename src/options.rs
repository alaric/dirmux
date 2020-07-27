use std::path::PathBuf;
use structopt::StructOpt;

/// This tool provides a convenient and fast interface to running commands across many directories,
/// managed by tags. This is especially useful when, for example, managing multiple git repositories
/// from a similar organisation.
///
/// The majority of the behaviour is exposed by specific subcommands which are illustrated below,
/// and all have their own help content. However, in addition to these explicit commands dirmux also
/// succinctly supports running any arbitrary shell command instead of the subcommand. As an
/// example:
///
/// $ dirmux du -sh .
///
/// This will evaluate all the size of all the directories you have tagged.
#[derive(Clone, Debug, PartialEq, StructOpt)]
pub struct Options {
    /// Which tag name to operate on
    #[structopt(short)]
    pub tag: Option<String>,

    /// How many concurrent jobs to run
    ///
    /// It might be necessary to tune this if your commands are heavily CPU dependent or use
    /// external, rate-limited resources.
    #[structopt(short, long, default_value = "10")]
    pub jobs: usize,

    #[structopt(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub enum Subcommands {
    /// Provides a compact git status
    ///
    /// This subcommand prints a summary of the git status of all the
    /// directories. This by default will skip printing anything for
    /// directories that are on their normal branch and have no
    /// outstanding status items (modified files, untracked changes).
    /// The goal is to have a quick oversight of all your outstanding
    /// changes waiting for a commit.
    Status(StatusOpts),

    /// Manipulate the tagged directories
    ///
    /// This provides a number of subcommands for manipulating which directories are tagged with
    /// which tags.
    Tag(TagOpts),

    /// Execute an arbitrary shell command
    ///
    /// This command is just here as a placeholder for the documentation, if the command doesn't
    /// overlap with any of dirmux's subcommand names, you may omit the 'exec' subcommand to have
    /// the same effect.
    Exec(ExecOpts),

    /// Any command is also allowed
    #[structopt(external_subcommand)]
    RawCommand(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, StructOpt)]
pub enum TagSubcommands {
    /// Add tag to this directory
    Add(TagAddOpts),
    /// Remove tag from this directory
    Remove(TagRemoveOpts),
    /// Garbage collect non-existing directories
    ///
    /// This option will check the existence of all the directories and remove tags from
    /// non-existing directories.
    Gc,
}

/// Test
#[derive(Clone, Debug, PartialEq, StructOpt)]
pub struct StatusOpts {}

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
