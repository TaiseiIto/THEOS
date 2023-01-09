extern crate regex;

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
    pub fn new(path: &path::PathBuf) -> Self {
        if !path.exists() {
            panic!("\"{}\" is not found.", path.display());
        }
        let path: path::PathBuf = path.to_path_buf();
        let name: String = match path.file_name() {
            Some(name) => match name.to_os_string().into_string() {
                Ok(name) => name,
                _ => String::from(""),
            },
            None => String::from(""),
        };
        let access_time = time::Time::get_access_time(&path);
        let change_time = time::Time::get_change_time(&path);
        let modification_time = time::Time::get_modification_time(&path);
        let content = FileOrDirectory::new(&path);
        Self {
            path,
            name,
            access_time,
            change_time,
            modification_time,
            content,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let regex = regex::Regex::new("^|\n").expect("Can't create a Regex.");
        write!(f, "object.path = {}\n", self.path.display())?;
        write!(f, "object.name = {}\n", self.name)?;
        let access_time: String = format!("{}", self.access_time);
        let access_time: String = regex.replace_all(&access_time, "$0object.access.");
        write!(f, "{}\n", access_time)?;
        let change_time: String = format!("{}", self.change_time);
        let change_time: String = regex.replace_all(&change_time, "$0object.change.");
        write!(f, "{}\n", change_time)?;
        let modification_time: String = format!("{}", self.modification_time);
        let modification_time: String = regex.replace_all(&modification_time, "$0object.modification.");
        write!(f, "{}\n", modification_time)?;
        let content: String = format!("{}", self.content);
        let content: String = regex.replace_all(&content, "$0object.content.");
        write!(f, "{}", content)
    }
}

#[derive(Debug)]
pub enum FileOrDirectory {
    File {
        bytes: Vec<u8>,
    },
    Directory {
        children: Vec<Object>,
    },
}

impl FileOrDirectory {
    fn new(path: &path::PathBuf) -> Self {
        if path.is_file() {
            let file = fs::File::open(&path).expect(&format!("\"{}\" is not found.", path.display()));
            let mut file = BufReader::new(file);
            let mut bytes = Vec::<u8>::new();
            file.read_to_end(&mut bytes).expect(&format!("Can't read \"{}\".", path.display()));
            Self::File {
                bytes,
            }
        } else if path.is_dir() {
            Self::Directory {
                children: {
                    match fs::read_dir(path) {
                        Ok(dir) => dir
                            .into_iter()
                            .filter_map(|dir| dir.ok())
                            .map(|dir| Object::new(&dir.path()))
                            .collect(),
                        _ => vec![],
                    }
                },
            }
        } else {
            panic!("\"{}\" is not a file or directory.", path.display());
        }
    }
}

impl fmt::Display for FileOrDirectory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileOrDirectory::File {
                bytes,
            } => write!(f, "file contents = {:x?}", bytes),
            FileOrDirectory::Directory {
                children,
            } => {
                for (i, child) in children.iter().enumerate() {
                    let regex = regex::Regex::new("^|\n").expect("Can't create a Regex.");
                    let child: String = format!("{}", child);
                    let child: String = regex.replace_all(&child, &format!("$0child[{}].", i) as &str);
                    write!(f, "{}", child)?;
                }
                write!(f, "")
            }
        }
    }
}

