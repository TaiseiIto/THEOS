use {
    std::{
        cell::{
            Ref,
            RefCell,
        },
        fmt,
        fs,
        path::PathBuf,
        rc::{
            Rc,
            Weak,
        },
    },
    super::{
        cluster,
        directory_entry,
    },
};

#[derive(Debug)]
pub enum Content {
    File {
        bytes: Vec<u8>,
    },
    Directory {
        children: RefCell<Vec<Rc<Node>>>,
        node: RefCell<Weak<Node>>,
        is_root: bool,
    },
}

impl Content {
    pub fn root(source: &PathBuf, volume_label: String, cluster_size: usize, root_directory_entries: usize) -> Self {
        if let Self::Directory {
            children,
            node,
            is_root: _,
        } = source.into() {
            for child in children.borrow().iter() {
                child.set_parent();
            }
            let node = RefCell::new(Weak::new());
            let is_root: bool = true;
            let root = Self::Directory {
                children,
                node,
                is_root,
            };
            let mut cluster = cluster::Clusters::new(cluster_size);
            root
        } else {
            panic!("Can't generate a root directory.");
        }
    }
}

impl fmt::Display for Content {
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
                node: _,
                is_root: _,
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

impl From<&PathBuf> for Content {
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
            let node = RefCell::new(Weak::new());
            let is_root: bool = false;
            Self::Directory {
                children,
                node,
                is_root,
            }
        } else {
            panic!("Can't find {}!", source.display())
        }
    }
}

impl Into<Vec<u8>> for &Content {
    fn into(self) -> Vec<u8> {
        match self {
            Content::File {
                bytes
            } => bytes.clone(),
            Content::Directory {
                children,
                node,
                is_root,
            } => {
                let mut directory_entries: Vec<&directory_entry::DirectoryEntry> = vec![];
                let node: Rc<Node> = node
                    .borrow()
                    .upgrade()
                    .expect("Can't convert a directory into bytes.");
                let current_directory_entry: &directory_entry::DirectoryEntry = &node.current_directory_entry;
                directory_entries.push(current_directory_entry);
                let parent: Rc<Node> = node.parent
                    .borrow()
                    .upgrade()
                    .expect("Can't convert a directory into bytes.");
                let parent_directory_entry: &directory_entry::DirectoryEntry = &parent.current_directory_entry;
                directory_entries.push(parent_directory_entry);
                let children: Ref<'_, Vec<Rc<Node>>> = children.borrow();
                let children_directory_entry: Vec<&directory_entry::DirectoryEntry> = children
                    .iter()
                    .map(|child| &child.directory_entry)
                    .collect();
                directory_entries.extend(children_directory_entry);
                directory_entry::DirectoryEntry::deduplicate(&directory_entries);
                let directory_entries: Vec<Vec<u8>> = directory_entries
                    .into_iter()
                    .map(|directory_entry| directory_entry.into())
                    .collect();
                directory_entries
                    .into_iter()
                    .flatten()
                    .collect()
            },
        }
    }
}

#[derive(Debug)]
pub struct Node {
    name: String,
    content: Content,
    directory_entry: directory_entry::DirectoryEntry,
    current_directory_entry: directory_entry::DirectoryEntry,
    parent_directory_entry: directory_entry::DirectoryEntry,
    parent: RefCell<Weak<Self>>,
}

impl Node {
    pub fn is_directory(&self) -> bool {
        match &self.content {
            Content::File {
                bytes: _,
            } => false,
            Content::Directory {
                children: _,
                node: _,
                is_root: _,
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
        if let Content::Directory {
            children,
            node,
            is_root: _,
        } = &self.clone().content {
            *node.borrow_mut() = Rc::downgrade(self);
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
        let content: Content = source.into();
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

impl Into<Vec<u8>> for &Node {
    fn into(self) -> Vec<u8> {
        let content: &Content = &self.content;
        content.into()
    }
}

