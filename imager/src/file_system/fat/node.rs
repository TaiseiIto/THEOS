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
    },
}

impl Content {
    pub fn read(directory_entry: &directory_entry::DirectoryEntry, clusters: &cluster::Clusters) -> (Self, Option<directory_entry::DirectoryEntry>, Option<directory_entry::DirectoryEntry>) {
        if let directory_entry::DirectoryEntry::ShortFileName {
            stem,
            extension,
            attribute,
            name_flags,
            created_time,
            accessed_time,
            written_time,
            cluster,
            size,
            long_file_name,
            checksum,
        } = directory_entry {
            let cluster: u32 = match *cluster.borrow() {
                Some(cluster) => cluster,
                None => panic!("Can't read a content."),
            };
            let mut bytes: Vec<u8> = clusters.cluster_chain_bytes(cluster);
            if attribute.is_directory() {
                let directory_entries: Vec<directory_entry::DirectoryEntry> = directory_entry::DirectoryEntry::read(&bytes);
                let current_directory_entry: Option<directory_entry::DirectoryEntry> = directory_entries
                    .iter()
                    .find(|directory_entry| directory_entry.is_current_directory_entry())
                    .map(|current_directory_entry| current_directory_entry.clone());
                let parent_directory_entry: Option<directory_entry::DirectoryEntry> = directory_entries
                    .iter()
                    .find(|directory_entry| directory_entry.is_parent_directory_entry())
                    .map(|parent_directory_entry| parent_directory_entry.clone());
                let children: Vec<Rc<Node>> = directory_entries
                    .into_iter()
                    .filter(|directory_entry| !directory_entry.is_current_directory_entry() && !directory_entry.is_parent_directory_entry())
                    .map(|directory_entry| Rc::new(Node::read(&directory_entry, clusters)))
                    .collect();
                let children: RefCell<Vec<Rc<Node>>> = RefCell::new(children);
                let node: RefCell<Weak<Node>> = RefCell::new(Weak::new());
                let content = Self::Directory {
                    children,
                    node,
                };
                (content, current_directory_entry, parent_directory_entry)
            } else {
                bytes.truncate(*size);
                let content = Self::File {
                    bytes,
                };
                let current_directory_entry: Option<directory_entry::DirectoryEntry> = None;
                let parent_directory_entry: Option<directory_entry::DirectoryEntry> = None;
                (content, current_directory_entry, parent_directory_entry)
            }
        } else {
            panic!("Can't read a content.")
        }
    }

    pub fn read_root(root_directory: &Vec<u8>, clusters: &cluster::Clusters) -> (Self, String) {
        let directory_entries: Vec<directory_entry::DirectoryEntry> = directory_entry::DirectoryEntry::read(root_directory);
        let volume_label: String = directory_entries
            .iter()
            .find(|directory_entry| if let directory_entry::DirectoryEntry::ShortFileName {
                stem: _,
                extension: _,
                attribute,
                name_flags: _,
                created_time: _,
                accessed_time: _,
                written_time: _,
                cluster: _,
                size: _,
                long_file_name: _,
                checksum: _,
            } = directory_entry {
                attribute.is_volume_id()
            } else {
                false
            })
            .expect("Can't find a volume label.")
            .get_name();
        let children: Vec<Node> = directory_entries
            .iter()
            .filter(|directory_entry| if let directory_entry::DirectoryEntry::ShortFileName {
                stem: _,
                extension: _,
                attribute,
                name_flags: _,
                created_time: _,
                accessed_time: _,
                written_time: _,
                cluster: _,
                size: _,
                long_file_name: _,
                checksum: _,
            } = directory_entry {
                !attribute.is_volume_id()
            } else {
                false
            })
            .map(|directory_entry| Node::read(directory_entry, clusters))
            .collect();
        let children: Vec<Rc<Node>> = children
            .into_iter()
            .map(|child| Rc::new(child))
            .collect();
        let children: RefCell<Vec<Rc<Node>>> = RefCell::new(children);
        for child in children.borrow().iter() {
            child.set_parent();
        }
        let node: RefCell<Weak<Node>> = RefCell::new(Weak::new());
        let root = Self::Directory {
            children,
            node,
        };
        (root, volume_label)
    }

    pub fn root(source: &PathBuf, volume_label: &str, cluster_size: usize) -> (Self, cluster::Clusters) {
        if let Self::Directory {
            children,
            node,
        } = source.into() {
            for child in children.borrow().iter() {
                child.set_parent();
            }
            let node = RefCell::new(Weak::new());
            let root = Self::Directory {
                children,
                node,
            };
            // Temporary clusters to determine cluster number of each node.
            let mut clusters = cluster::Clusters::new(cluster_size);
            root.write_root(&mut clusters);
            // Correct clusters.
            let mut clusters = cluster::Clusters::new(cluster_size);
            root.write_root(&mut clusters);
            (root, clusters)
        } else {
            panic!("Can't generate a root directory.");
        }
    }

    pub fn root_into_bytes(&self, volume_label: &str, root_directory_entries: usize) -> Vec<u8> {
        let mut directory_entries: Vec<&directory_entry::DirectoryEntry> = vec![];
        let volume_label = directory_entry::DirectoryEntry::volume_label(volume_label);
        directory_entries.push(&volume_label);
        let children: Ref<'_, Vec<Rc<Node>>> = if let Self::Directory {
            children,
            node: _,
        } = self {
            children.borrow()
        } else {
            panic!("Can't convert a root directory into bytes.");
        };
        let children: Vec<&directory_entry::DirectoryEntry> = children
            .iter()
            .map(|node| &node.directory_entry)
            .collect();
        directory_entries.extend(children);
        directory_entry::DirectoryEntry::deduplicate(&directory_entries);
        let directory_entries: Vec<Vec<u8>> = directory_entries
            .into_iter()
            .map(|directory_entry| directory_entry.into())
            .collect();
        let mut directory_entries: Vec<u8> = directory_entries
            .into_iter()
            .flatten()
            .collect();
        let size: usize = root_directory_entries * directory_entry::DIRECTORY_ENTRY_SIZE;
        let blank: u8 = 0x00;
        directory_entries.resize(size, blank);
        directory_entries
    }

    fn write_clusters(&self, clusters: &mut cluster::Clusters) {
        let bytes: Vec<u8> = self.into();
        let blank: u8 = 0x00;
        clusters.append(&bytes, blank);
        if let Self::Directory {
            children,
            node: _,
        } = self {
            for child in children.borrow().iter() {
                child.write_clusters(clusters);
            }
        }
    }

    fn write_root(&self, clusters: &mut cluster::Clusters) {
        if let Self::Directory {
            children,
            node: _,
        } = self {
            for child in children.borrow().iter() {
                child.write_clusters(clusters);
            }
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
            Self::Directory {
                children,
                node,
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
            } => {
                let mut directory_entries: Vec<&directory_entry::DirectoryEntry> = vec![];
                let node: Rc<Node> = node
                    .borrow()
                    .upgrade()
                    .expect("Can't convert a directory into bytes.");
                let current_directory_entry: &directory_entry::DirectoryEntry = &node.current_directory_entry;
                directory_entries.push(current_directory_entry);
                let parent: Option<Rc<Node>> = node
                    .parent
                    .borrow()
                    .upgrade();
                let parent_directory_entry: directory_entry::DirectoryEntry = match parent {
                    Some(parent) => parent.parent_directory_entry.clone(),
                    None => directory_entry::DirectoryEntry::parent_root_directory_entry(),
                };
                directory_entries.push(&parent_directory_entry);
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
            } => true,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn read(directory_entry: &directory_entry::DirectoryEntry, clusters: &cluster::Clusters) -> Self {
        let name: String = directory_entry.get_name();
        let (content, current_directory_entry, parent_directory_entry): (Content, Option<directory_entry::DirectoryEntry>, Option<directory_entry::DirectoryEntry>) = Content::read(directory_entry, clusters);
        let directory_entry: directory_entry::DirectoryEntry = directory_entry.clone();
        let current_directory_entry: directory_entry::DirectoryEntry = match current_directory_entry {
            Some(current_directory_entry) => current_directory_entry,
            None => directory_entry.current_directory_entry(),
        };
        let parent_directory_entry: directory_entry::DirectoryEntry = match parent_directory_entry {
            Some(parent_directory_entry) => parent_directory_entry,
            None => directory_entry.parent_directory_entry(),
        };
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

    fn write_clusters(&self, clusters: &mut cluster::Clusters) {
        let cluster_number: u32 = match &self.content {
            Content::File {
                bytes,
            } => match bytes.len() {
                0 => 0,
                _ => clusters.next_cluster_number(),
            },
            Content::Directory {
                children: _,
                node: _,
            } => clusters.next_cluster_number(),
        };
        self.directory_entry.set_cluster(cluster_number);
        self.current_directory_entry.set_cluster(cluster_number);
        self.parent_directory_entry.set_cluster(cluster_number);
        self.content.write_clusters(clusters);
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

