use {
    std::{
        cell::RefCell,
        char,
        fmt,
        fs,
        path::PathBuf,
        rc::{
            Rc,
            Weak,
        },
    },
    super::super::super::time,
};

#[derive(Debug)]
pub struct Node {
    content: FileOrDirectory,
    last_accessed_time: time::Time,
    last_changed_time: time::Time,
    last_modified_time: time::Time,
    name: String,
    parent: RefCell<Weak<Self>>,
    hidden: bool,
    read_only: bool,
    system: bool,
}

impl Node {
    pub fn new(path: &PathBuf) -> Rc<Self> {
        let content = FileOrDirectory::new(path);
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
            parent,
            hidden,
            read_only,
            system,
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
        let string: Vec<String> = vec![
            name,
            last_accessed_time,
            last_changed_time,
            last_modified_time,
            hidden,
            read_only,
            system,
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
    pub fn new(path: &PathBuf) -> Self {
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
