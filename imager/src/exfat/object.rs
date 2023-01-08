use std::path;

#[derive(Debug)]
pub struct Object {
    path: path::PathBuf,
    content: FileOrDirectory,
}

#[derive(Debug)]
enum FileOrDirectory {
    File,
    Directory,
}

impl Object {
    pub fn new(path: &path::PathBuf) -> Self {
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

