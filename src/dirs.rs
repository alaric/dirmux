use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileFormat {
    tags: HashMap<String, Vec<PathBuf>>,
}

impl FileFormat {
    pub fn blank() -> Self {
        FileFormat {
            tags: HashMap::new()
        }
    }

    pub fn add(&mut self, tag: String, path: &PathBuf) {
        self.tags.entry(tag).or_insert(vec![]).insert(0, path.clone());
    }

    pub fn remove(&mut self, tag: String, path: &PathBuf) {
        self.tags.entry(tag).or_insert(vec![]).retain(|x| path != x);
        self.tags.retain(|_, v| v.len() > 0);
    }

    pub fn retain<F>(&mut self, f: F) where F: Fn(&PathBuf) -> bool {
        for (_, v) in &mut self.tags {
            v.retain(|x| f(x));
        }
        self.tags.retain(|_, v| v.len() > 0);
    }
}

pub fn read_file(file: &Path) -> Result<FileFormat> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

pub fn write_file(file: &FileFormat, filename: &Path) -> Result<()> {
    let out = File::create(filename)?;
    serde_json::to_writer_pretty(out, file)?;
    Ok(())
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
