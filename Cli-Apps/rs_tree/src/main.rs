use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use thiserror::Error;

pub type Result<T> = anyhow::Result<T, anyhow::Error>;


#[derive(Error, Debug)]
pub enum TreeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}


struct RegularFile {
    name: String,
    size: usize
}

impl RegularFile {
    fn new(name: String, size: usize) -> Self {
        Self {
            name,
            size
        }
    }
}

struct Directory {
    name: String,
    len: usize,
    files: Vec<RegularFile>,
    directories: Vec<Directory>
}

impl Directory {
    // one possible implementation of walking a directory only visiting files
    fn from_path(dir: &Path) -> Result<Self> {
        let mut directories: Vec<Directory> = Vec::new();
        let mut files: Vec<RegularFile> = Vec::new();

        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let metadata = fs::metadata(&entry.path())?;
                if metadata.is_dir() {
                    todo!()
                } else {
                    todo!()
                }
            }
        }
        todo!()
        
    }
}


fn main() -> io::Result<()> {
    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();
    Ok(())
}


