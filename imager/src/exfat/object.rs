use std::{
    fmt,
    fs,
    io::{
        BufReader,
        Read,
    },
    path,
};
use super::super::time;

#[derive(Debug)]
pub struct Object {
    path: path::PathBuf,
    name: String,
    access_time: time::Time,
    change_time: time::Time,
    modification_time: time::Time,
    content: FileOrDirectory,
}

impl Object {
    pub fn new(path: path::PathBuf) -> Self {
        if !path.exists() {
            panic!("\"{}\" is not found.", path.display());
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
            access_time: time::Time::get_access_time(&path),
            change_time: time::Time::get_change_time(&path),
            modification_time: time::Time::get_modification_time(&path),
            content: if path.is_file() {
                let file = fs::File::open(&path).expect(&format!("\"{}\" is not found.", path.display()));
                let mut file = BufReader::new(file);
                let mut bytes = Vec::<u8>::new();
                file.read_to_end(&mut bytes).expect(&format!("Can't read \"{}\".", path.display()));
                FileOrDirectory::File {
                    bytes,
                }
            } else if path.is_dir() {
                FileOrDirectory::Directory {
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
            } else {
                panic!("\"{}\" is not a file or directory.", path.display());
            },
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "object.path = {}\n", self.path.display())?;
        write!(f, "object.name = {}\n", self.name)?;
        let access_time: String = format!("{}", self.access_time)
            .replace("time", "object.access_time");
        let change_time: String = format!("{}", self.change_time)
            .replace("time", "object.change_time");
        let modification_time: String = format!("{}", self.modification_time)
            .replace("time", "object.modification_time");
        write!(f, "{}\n", access_time)?;
        write!(f, "{}\n", change_time)?;
        write!(f, "{}\n", modification_time)?;
        write!(f, "object.content = {}", self.content)
    }
}

#[derive(Debug)]
enum FileOrDirectory {
    File {
        bytes: Vec<u8>,
    },
    Directory {
        children: Vec<Object>,
    },
}

impl fmt::Display for FileOrDirectory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileOrDirectory::File {
                bytes,
            } => write!(f, "File {:x?}", bytes),
            FileOrDirectory::Directory {
                children,
            } => {
                write!(f, "Directory\n")?;
                for child in children {
                    write!(f, "{}\n", child)?;
                }
                write!(f, "")
            }
        }
    }
}

