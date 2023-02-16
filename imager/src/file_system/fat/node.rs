use {
    std::{
        cell::RefCell,
        fmt,
        fs,
        path::PathBuf,
        rc::{
            Rc,
            Weak,
        },
    },
    super::directory_entry,
};

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
    pub fn root(source: &PathBuf) -> Self {
        if let Self::Directory {
            children,
        } = source.into() {
            for child in children.borrow().iter() {
                child.set_parent();
            }
            Self::Directory {
                children,
            }
        } else {
            panic!("Can't generate a root directory.");
        }
    }
}

impl fmt::Display for FileOrDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string: String = match self {
            Self::File {
                bytes,
            } => bytes
                .chunks(0x10)
                .map(|bytes| bytes
                    .into_iter()
                    .map(|byte| (format!("{:02x} ", byte), if 0x20 <= *byte && *byte <= 0x7f {
                        char::from(*byte)
                    } else {
                        ' '
                    }))
                    .map(|(hex, c)| {
                        let c: char = match c {
                            '\n' |
                            '\t' |
                            '\r' => ' ',
                            c => c,
                        };
                        (hex, c)
                    })
                    .fold((String::new(), String::new()), |(hex_line, mut c_line), (hex, c)| {
                        c_line.push(c);
                        (hex_line + &hex, c_line)
                    }))
                .map(|(mut hex_line, c_line)| {
                    while hex_line.len() < 0x30 {
                        hex_line.push(' ');
                    }
                    hex_line + &c_line
                })
                .collect::<Vec<String>>()
                .join("\n"),
            Self::Directory {
                children,
            } => children
                .borrow()
                .iter()
                .map(|child| format!("{}", child))
                .collect::<Vec<String>>()
                .join("\n"),
        };
        write!(f, "{}", string)
    }
}

impl From<&PathBuf> for FileOrDirectory {
    fn from(source: &PathBuf) -> Self {
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
                        let source: &PathBuf = &directory.path();
                        let node: Node = source.into();
                        Rc::new(node)
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

#[derive(Debug)]
pub struct Node {
    name: String,
    content: FileOrDirectory,
    directory_entry: directory_entry::DirectoryEntry,
    current_directory_entry: directory_entry::DirectoryEntry,
    parent_directory_entry: directory_entry::DirectoryEntry,
    parent: RefCell<Weak<Self>>,
}

impl Node {
    pub fn is_directory(&self) -> bool {
        match &self.content {
            FileOrDirectory::File {
                bytes
            } => false,
            FileOrDirectory::Directory {
                children,
            } => true,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn path(&self) -> PathBuf {
        let mut path: PathBuf = match self.parent.borrow().upgrade() {
            Some(parent) => parent.path(),
            None => {
                let root = String::from("/");
                PathBuf::from(root)
            },
        };
        path.push(&self.name);
        path
    }

    fn set_parent(self: &Rc<Self>) {
        if let FileOrDirectory::Directory {
            children,
        } = &self.clone().content {
            for child in children.borrow_mut().iter_mut() {
                child.set_parent();
                *child
                    .clone()
                    .parent
                    .borrow_mut() = Rc::downgrade(self);
            }
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path: String = self
            .path()
            .to_str()
            .expect("Can't print a node.")
            .to_string();
        let content: String = format!("{}", self.content);
        let directory_entry: String = format!("{}", self.directory_entry)
            .lines()
            .map(|line| format!("directory_entry.{}", line))
            .collect::<Vec<String>>()
            .join("\n");
        let current_directory_entry: String = format!("{}", self.current_directory_entry)
            .lines()
            .map(|line| format!("current_directory_entry.{}", line))
            .collect::<Vec<String>>()
            .join("\n");
        let parent_directory_entry: String = format!("{}", self.parent_directory_entry)
            .lines()
            .map(|line| format!("parent_directory_entry.{}", line))
            .collect::<Vec<String>>()
            .join("\n");
        let directory_entries: Vec<String> = vec![
            directory_entry,
            current_directory_entry,
            parent_directory_entry,
        ];
        let directory_entries: String = directory_entries
            .join("\n")
            .lines()
            .map(|line| format!("{} {}", path, line))
            .collect::<Vec<String>>()
            .join("\n");
        let elements: Vec<String> = vec![
            directory_entries,
            content,
        ];
        let string: String = elements
            .into_iter()
            .filter(|element| 0 < element.len())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", string)
    }
}

impl From<&PathBuf> for Node {
    fn from(source: &PathBuf) -> Self {
        let name: String = source
            .file_name()
            .expect(&format!("Can't get a basename of {}!", source.display()))
            .to_str()
            .expect(&format!("Can't get a basename of {}!", source.display()))
            .to_string();
        let content: FileOrDirectory = source.into();
        let directory_entry: directory_entry::DirectoryEntry = source.into();
        let current_directory_entry: directory_entry::DirectoryEntry = directory_entry.current_directory_entry();
        let parent_directory_entry: directory_entry::DirectoryEntry = directory_entry.parent_directory_entry();
        let parent = RefCell::new(Weak::new());
        Self {
            name,
            content,
            directory_entry,
            current_directory_entry,
            parent_directory_entry,
            parent,
        }
    }
}

