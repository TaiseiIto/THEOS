use std::{
    cell::RefCell,
    fs,
    path::PathBuf,
    rc::Rc,
};

#[derive(Debug)]
pub struct Node {
    content: FileOrDirectory,
    name: String,
}

impl Node {
    pub fn new(path: &PathBuf) -> Rc<Self> {
        let content = FileOrDirectory::new(path);
        let name: String = path
            .file_name()
            .expect("Can't generate a node.")
            .to_str()
            .expect("Can't generate a node.")
            .to_string();
        Rc::new(Self {
            content,
            name,
        })
    }
}

#[derive(Debug)]
pub enum FileOrDirectory {
    File,
    Directory {
        children: RefCell<Vec<Rc<Node>>>,
    },
}

impl FileOrDirectory {
    pub fn new(path: &PathBuf) -> Self {
        if path.is_file() {
            Self::File
        } else if path.is_dir() {
            let children: Vec<Rc<Node>> = match fs::read_dir(path) {
                Ok(directory) => directory
                    .into_iter()
                    .filter_map(|directory| directory.ok())
                    .map(|directory| {
                        let directory: &PathBuf = &directory.path();
                        Node::new(directory)
                    })
                    .collect(),
                _ => vec![],
            };
            let children: RefCell<Vec<Rc<Node>>> = RefCell::new(children);
            Self::Directory {
                children,
            }
        } else {
            panic!("{} is not found.", path.display())
        }
    }
}

