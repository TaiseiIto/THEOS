use std::{
    fmt,
    fs,
    path,
};

#[derive(Debug)]
pub struct Object {
    path: path::PathBuf,
    name: String,
    content: FileOrDirectory,
    children: Vec<Object>,
}

impl Object {
    pub fn new(path: path::PathBuf) -> Self {
        if !path.exists() {
            panic!("No such a file or directory \"{}\".", path.display());
        }
        Self {
            path: path.to_path_buf(),
            name: match path.file_name() {
                Some(name) => match name.to_os_string().into_string() {
                    Ok(name) => name,
                    _ => String::from(""),
                },
                None => String::from(""),
            },
            content: if path.is_file() {
                FileOrDirectory::File
            } else if path.is_dir() {
                FileOrDirectory::Directory
            } else {
                panic!("\"{}\" is not a file or directory.", path.display());
            },
            children: {
                match fs::read_dir(path) {
                    Ok(dir) => dir
                        .into_iter()
                        .filter_map(|dir| dir.ok())
                        .map(|dir| Self::new(dir.path()))
                        .collect(),
                    _ => vec![],
                }
            },
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "object.path = {}\n", self.path.display())?;
        write!(f, "object.name = {}\n", self.name)?;
        write!(f, "object.content = {}\n", self.content)?;
        for (i, child) in self.children.iter().enumerate() {
            let child = format!("{}", child)
                .replace("object", &format!("object.child[{}]", i));
            write!(f, "{}\n", child)?;
        }
        write!(f, "")
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

