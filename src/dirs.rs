use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileFormat {
    tags: HashMap<String, Vec<PathBuf>>,
}

pub fn read_file(file: &Path) -> Result<FileFormat> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

pub fn get_dirs(file: FileFormat, filters: Vec<&str>) -> Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();

    if filters.len() == 0 {
        for i in file.tags {
            dirs.extend_from_slice(&i.1);
        }
    }
    else {
        for i in filters {
            match file.tags.get(i) {
                Some(res) => dirs.extend_from_slice(res),
                None => {}
            }
        }
    }

    dirs.sort();
    dirs.dedup();
    Ok(dirs)
}
