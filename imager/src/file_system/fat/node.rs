use {
    std::{
        cell::RefCell,
        char,
        convert::Into,
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
        super::super::time,
    },
};

#[derive(Debug)]
pub struct Node {
    content: FileOrDirectory,
    last_accessed_time: time::Time,
    last_changed_time: time::Time,
    last_modified_time: time::Time,
    name: String,
    hidden: bool,
    read_only: bool,
    system: bool,
    first_cluster: Option<u32>,
    number_of_clusters: usize,
    parent: RefCell<Weak<Self>>,
}

impl Node {
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn is_system(&self) -> bool {
        self.system
    }

    pub fn is_directory(&self) -> bool {
        self.content.is_directory()
    }

    pub fn new(path: &PathBuf, clusters: &cluster::Clusters) -> Rc<Self> {
        let content = FileOrDirectory::new(path, clusters);
        let cluster_size: usize = clusters.cluster_size();
        let first_cluster: Option<u32> = None;
        let number_of_clusters: usize = content.number_of_clusters(cluster_size);
        let last_accessed_time = time::Time::last_accessed_time(path);
        let last_changed_time = time::Time::last_changed_time(path);
        let last_modified_time = time::Time::last_modified_time(path);
        let name: String = path
            .file_name()
            .expect("Can't generate a node.")
            .to_str()
            .expect("Can't generate a node.")
            .to_string();
        let parent = RefCell::new(Weak::new());
        let hidden = false;
        let read_only = true;
        let system = true;
        let node: Rc<Self> = Rc::new(Self {
            content,
            last_accessed_time,
            last_changed_time,
            last_modified_time,
            name,
            hidden,
            read_only,
            system,
            first_cluster,
            number_of_clusters,
            parent,
        });
        if let FileOrDirectory::Directory {
            children,
        } = &node.content {
            for child in children.borrow_mut().iter_mut() {
                *child.parent.borrow_mut() = Rc::downgrade(&node);
            }
        }
        const FIRST_CLUSTER: u32 = 2;
        node.set_first_cluster(FIRST_CLUSTER).0
    }

    fn get_path(&self) -> PathBuf {
        let mut path: PathBuf = match self.parent.borrow().upgrade() {
            Some(parent) => parent.get_path(),
            None => PathBuf::from("/"),
        };
        path.push(&self.name);
        path
    }

    fn long_name(&self) -> Vec<u16> {
        self.name
            .encode_utf16()
            .collect()
    }

    fn number_of_directory_entries(&self) -> usize {
        const LETTERS_PER_LONG_NAME_ENTRY: usize = 13;
        self.long_name().len() / LETTERS_PER_LONG_NAME_ENTRY + 1
    }

    fn set_first_cluster(&self, first_cluster: u32) -> (Rc<Self>, u32) {
        let Self {
            content,
            last_accessed_time,
            last_changed_time,
            last_modified_time,
            name,
            hidden,
            read_only,
            system,
            first_cluster: _,
            number_of_clusters,
            parent,
        } = self;
        let next_cluster: u32 = first_cluster + *number_of_clusters as u32;
        let (content, next_cluster): (FileOrDirectory, u32) = content.set_first_cluster(next_cluster);
        let first_cluster: Option<u32> = Some(first_cluster);
        (Rc::new(Self {
            content,
            last_accessed_time: *last_accessed_time,
            last_changed_time: *last_changed_time,
            last_modified_time: *last_modified_time,
            name: name.clone(),
            hidden: *hidden,
            read_only: *read_only,
            system: *system,
            first_cluster,
            number_of_clusters: *number_of_clusters,
            parent: parent.clone(),
        }), next_cluster)
    }
}

impl Into<Vec<u8>> for &Node {
    fn into(self) -> Vec<u8> {
        (&self.content).into()
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content: String = format!("{}", self.content);
        let last_accessed_time: String = format!("last_accessed_time: {}", self.last_accessed_time);
        let last_changed_time: String = format!("last_changed_time: {}", self.last_changed_time);
        let last_modified_time: String = format!("last_modified_time: {}", self.last_modified_time);
        let name: String = format!("{}", self.get_path().display());
        let hidden: String = format!("hidden: {}", self.hidden);
        let read_only: String = format!("read_only: {}", self.read_only);
        let system: String = format!("system: {}", self.system);
        let first_cluster: String = format!("first_cluster: {}", match self.first_cluster{
            Some(first_cluster) => format!("{}", first_cluster),
            None => "Undefined".to_string(),
        });
        let number_of_clusters: String = format!("number_of_clusters: {}", self.number_of_clusters);
        let string: Vec<String> = vec![
            name,
            last_accessed_time,
            last_changed_time,
            last_modified_time,
            hidden,
            read_only,
            system,
            first_cluster,
            number_of_clusters,
            content,
        ];
        let string: String = string
            .into_iter()
            .fold(String::new(), |string, element| string + "\n" + &element);
        write!(f, "{}", string)
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
    pub fn is_directory(&self) -> bool {
        match self {
            Self::File {
                bytes: _,
            } => false,
            Self::Directory {
                children: _,
            } => true,
        }
    }

    pub fn new(path: &PathBuf, clusters: &cluster::Clusters) -> Self {
        if path.is_file() {
            let mut bytes: Vec<u8> = fs::read(path).expect(&format!("Can't read {}!", path.display()));
            Self::File {
                bytes,
            }
        } else if path.is_dir() {
            let children: Vec<Rc<Node>> = match fs::read_dir(path) {
                Ok(directory) => directory
                    .into_iter()
                    .filter_map(|directory| directory.ok())
                    .map(|directory| {
                        let directory: &PathBuf = &directory.path();
                        Node::new(directory, clusters)
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

    fn number_of_clusters(&self, cluster_size: usize) -> usize {
        let length: usize = match self {
            Self::File {
                bytes,
            } => bytes.len(),
            Self::Directory {
                children,
            } => {
                let number_of_directory_entries: usize = children
                    .borrow()
                    .iter()
                    .map(|child| child.number_of_directory_entries())
                    .sum::<usize>() + 2;
                const DIRECTORY_ENTRY_LENGTH: usize = 0x20;
                number_of_directory_entries * DIRECTORY_ENTRY_LENGTH
            }
        };
        (length + cluster_size - 1) / cluster_size
    }

    fn set_first_cluster(&self, first_cluster: u32) -> (Self, u32) {
        match self {
            Self::File {
                bytes,
            } => (Self::File {
                bytes: bytes.clone(),
            }, first_cluster),
            Self::Directory {
                children,
            } => {
                let mut first_cluster: u32 = first_cluster;
                let mut new_children: Vec<Rc<Node>> = vec![];
                for child in children.clone().into_inner().into_iter() {
                    let (child, next_cluster): (Rc<Node>, u32) = child.set_first_cluster(first_cluster);
                    new_children.push(child);
                    first_cluster = next_cluster;
                }
                let children: RefCell<Vec<Rc<Node>>> = RefCell::new(new_children);
                (Self::Directory {
                    children,
                }, first_cluster)
            },
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
                    hex_line + &c_line + "\n"
                })
                .fold(String::new(), |string, line| string + &line),
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

impl Into<Vec<u8>> for &FileOrDirectory {
    fn into(self) -> Vec<u8> {
        match self {
            FileOrDirectory::File {
                bytes,
            } => bytes.clone(),
            FileOrDirectory::Directory {
                children,
            } => vec![],
        }
    }
}

