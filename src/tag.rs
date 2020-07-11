use crate::dirs::write_file;
use crate::dirs::FileFormat;
use crate::options::TagSubcommands::*;
use crate::options::*;
use anyhow::Result;
use std::path::PathBuf;

pub fn handle(tagopts: &TagOpts, filename: &PathBuf, file: &FileFormat) -> Result<()> {
    let dir = std::env::current_dir()?;
    let new_file = match &tagopts.action {
        Add(opts) => add(file, opts, &dir),
        Remove(opts) => remove(file, opts, &dir),
        Gc => gc(file),
    }?;
    write_file(&new_file, filename)?;
    Ok(())
}

fn add(file: &FileFormat, opts: &TagAddOpts, cwd: &PathBuf) -> Result<FileFormat> {
    let mut f = file.clone();
    let dir = opts.path.clone().unwrap_or(cwd.to_owned());
    f.add(opts.tag.clone(), &dir);
    Ok(f)
}

fn remove(file: &FileFormat, opts: &TagRemoveOpts, cwd: &PathBuf) -> Result<FileFormat> {
    let mut f = file.clone();
    let dir = opts.path.clone().unwrap_or(cwd.to_owned());
    f.remove(opts.tag.clone(), &dir);
    Ok(f)
}

fn gc(file: &FileFormat) -> Result<FileFormat> {
    let mut f = file.clone();
    f.retain(|x| x.is_dir());
    Ok(f)
}

#[test]
fn test_add() -> Result<()> {
    let opts = TagAddOpts {
        tag: String::from("dev"),
        path: None,
    };
    let mut prior = FileFormat::blank();
    let dir = std::env::current_dir()?;
    let subsequent = add(&prior, &opts, &dir)?;

    assert_ne!(prior, subsequent);
    prior.add(String::from("dev"), &dir);
    assert_eq!(prior, subsequent);
    Ok(())
}

#[test]
fn test_add_exact() -> Result<()> {
    let opts = TagAddOpts {
        tag: String::from("dev"),
        path: Some(PathBuf::from("/dev/null")),
    };
    let mut prior = FileFormat::blank();
    let dir = std::env::current_dir()?;
    let subsequent = add(&prior, &opts, &dir)?;

    assert_ne!(prior, subsequent);
    prior.add(String::from("dev"), &opts.path.unwrap());
    assert_eq!(prior, subsequent);
    Ok(())
}

#[test]
fn test_remove() -> Result<()> {
    let opts_add = TagAddOpts {
        tag: String::from("dev"),
        path: None,
    };
    let opts_rem = TagRemoveOpts {
        tag: String::from("dev"),
        path: None,
    };
    let prior = FileFormat::blank();
    let dir = std::env::current_dir()?;
    let subsequent = add(&prior, &opts_add, &dir)?;
    assert_ne!(prior, subsequent);
    let subsequent = remove(&subsequent, &opts_rem, &dir)?;
    assert_eq!(prior, subsequent);
    Ok(())
}

#[test]
fn test_gc() -> Result<()> {
    let mut expected = FileFormat::blank();
    let dir = std::env::current_dir()?;
    expected.add(String::from("exists"), &dir);
    let mut filled = expected.clone();
    filled.add(
        String::from("non-existing"),
        &PathBuf::from("/dev/does/not/exist"),
    );
    assert_ne!(expected, filled);
    let result = gc(&filled)?;
    assert_eq!(expected, result);
    Ok(())
}
