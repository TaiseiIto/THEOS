use std::{
    cell::RefCell,
    fmt,
    fs,
    path::PathBuf,
    rc::{
        Rc,
        Weak,
    },
};

#[derive(Debug)]
pub struct Node {
    content: FileOrDirectory,
    name: String,
    parent: RefCell<Weak<Self>>,
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
        let parent = RefCell::new(Weak::new());
        let node: Rc<Self> = Rc::new(Self {
            content,
            name,
            parent,
        });
        if let FileOrDirectory::Directory {
            children,
        } = &node.content {
            for child in children.borrow_mut().iter_mut() {
                *child.parent.borrow_mut() = Rc::downgrade(&node);
            }
        }
        node
    }

    fn get_path(&self) -> PathBuf {
        let mut path: PathBuf = match self.parent.borrow().upgrade() {
            Some(parent) => parent.get_path(),
            None => PathBuf::new(),
        };
        path.push(&self.name);
        path
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content: String = format!("{}", self.content);
        let name: String = format!("{}\n", self.get_path().display());
        let string: Vec<String> = vec![
            name,
            content,
        ];
        let string: String = string
            .into_iter()
            .fold(String::new(), |string, element| string + &element);
        write!(f, "{}", string)
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

impl fmt::Display for FileOrDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string: String = match self {
            Self::File => format!(""),
            Self::Directory {
                children
            } => children
                .borrow()
                .iter()
                .map(|child| format!("{}", child))
                .fold(String::new(), |string, child| string + &child)
        };
        write!(f, "{}", string)
    }
}

