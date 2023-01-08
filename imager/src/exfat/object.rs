use std::{
    fmt,
    path,
};

#[derive(Debug)]
pub struct Object {
    path: path::PathBuf,
    content: FileOrDirectory,
}

impl Object {
    pub fn new(path: path::PathBuf) -> Self {
        if !path.exists() {
            panic!("No such a file or directory \"{}\".", path.display());
        }
        Self {
            path: path.to_path_buf(),
            content: if path.is_file() {
                FileOrDirectory::File
            } else if path.is_dir() {
                FileOrDirectory::Directory
            } else {
                panic!("\"{}\" is not a file or directory.", path.display());
            },
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "object.path = {}\n", self.path.display())?;
        write!(f, "object.content = {}", self.content)
    }
}

#[derive(Debug)]
enum FileOrDirectory {
    File,
    Directory,
}

impl fmt::Display for FileOrDirectory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileOrDirectory::File => write!(f, "File"),
            FileOrDirectory::Directory => write!(f, "Directory"),
        }
    }
}

