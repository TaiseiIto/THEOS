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
    pub fn root(root: &PathBuf) -> Rc<Self> {
        let content = FileOrDirectory::new(root);
        let name: String = "".to_string();
        let root = Self {
            content,
            name,
        };
        let root: Rc<Self> = Rc::new(root);
        root
    }

    fn new(source: &PathBuf) -> Rc<Self> {
        let content = FileOrDirectory::new(source);
        let name: String = source
            .file_name()
            .expect(&format!("Can't get a basename of {}!", source.display()))
            .to_str()
            .expect(&format!("Can't get a basename of {}!", source.display()))
            .to_string();
        let node = Self {
            content,
            name,
        };
        let node: Rc<Self> = Rc::new(node);
        node
    }
}

#[derive(Debug)]
pub enum FileOrDirectory {
    File {
        bytes: Vec<u8>,
    },
    Directory {
        children: RefCell<Vec<Rc<Node>>>,
    },
}

impl FileOrDirectory {
    pub fn new(source: &PathBuf) -> Self {
        if source.is_file() {
            let bytes: Vec<u8> = fs::read(source).expect(&format!("Can't read {}!", source.display()));
            Self::File {
                bytes,
            }
        } else if source.is_dir() {
            let children: Vec<Rc<Node>> = match fs::read_dir(source) {
                Ok(directory) => directory
                    .into_iter()
                    .filter_map(|directory| directory.ok())
                    .map(|directory| {
                        let source: PathBuf = directory.path();
                        Node::new(&source)
                    })
                    .collect(),
                _ => panic!("Can't read a directory {}!", source.display()),
            };
            let children: RefCell<Vec<Rc<Node>>> = RefCell::new(children);
            Self::Directory {
                children,
            }
        } else {
            panic!("Can't find {}!", source.display())
        }
    }
}

