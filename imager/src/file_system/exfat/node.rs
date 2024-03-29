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
        }
    },
    super::{
        allocation_bitmap,
        boot_sector,
        cluster,
        directory_entry,
        fat,
        super::super::{
            guid,
            rand,
        },
        upcase_table,
    },
};

#[derive(Clone, Debug)]
pub enum Content {
    File {
        bytes: Vec<u8>,
    },
    Directory {
        children: RefCell<Vec<Rc<Node>>>,
        directory_entries: Vec<directory_entry::DirectoryEntry>,
    },
}

impl Content {
    pub fn allocation_bitmap(&self, clusters: &cluster::Clusters) -> allocation_bitmap::AllocationBitmap {
        if let Self::Directory {
            children: _,
            directory_entries,
        } = self {
            directory_entries
                .iter()
                .find_map(|directory_entry| if let directory_entry::DirectoryEntry::AllocationBitmap {
                    bitmap_identifier: _,
                    first_cluster,
                    data_length: _,
                } = directory_entry {
                    Some(clusters.allocation_bitmap(*first_cluster))
                } else {
                    None
                })
                .expect("Can't get an allocation bitmap.")
        } else {
            panic!("Can't get an allocation bitmap.")
        }
    }

    fn new(
        source: &PathBuf,
        destination: &PathBuf,
        is_root: bool,
        boot_sector: &boot_sector::BootSector,
        clusters: &mut cluster::Clusters,
        upcase_table: &upcase_table::UpcaseTable,
        has_volume_guid: bool,
        rand_generator: &mut rand::Generator,
    ) -> (Self, u32, usize) {
        if source.is_file() {
            let bytes: Vec<u8> = fs::read(source).expect(&format!("Can't read {}!", source.display()));
            let length: usize = bytes.len();
            let first_cluster: u32 = clusters.append(&bytes, 0);
            let file = Self::File {
                bytes,
            };
            (file, first_cluster, length)
        } else if source.is_dir() {
            let children: Vec<Rc<Node>> = match fs::read_dir(source) {
                Ok(directory) => directory
                    .into_iter()
                    .filter_map(|directory| directory.ok())
                    .map(|directory| {
                        let source: &PathBuf = &directory.path();
                        let destination: &mut PathBuf = &mut destination.to_path_buf();
                        destination.push(source.file_name().expect("Can't create a file or directory."));
                        Node::new(source, destination, false, boot_sector, clusters, upcase_table, has_volume_guid, rand_generator)
                    })
                    .collect(),
                _ => vec![],
            };
            let children: RefCell<Vec<Rc<Node>>> = RefCell::new(children);
            let mut directory_entries: Vec<directory_entry::DirectoryEntry> = children
                .borrow()
                .iter()
                .map(|object| object.directory_entry.clone().expect("Can't create a file or directory."))
                .collect();
            let upcase_table: Option<directory_entry::DirectoryEntry> = if is_root {
                Some(directory_entry::DirectoryEntry::upcase_table(upcase_table, clusters))
            } else {
                None
            };
            match upcase_table {
                Some(upcase_table) => directory_entries.push(upcase_table),
                None => (),
            }
            let volume_label: Option<directory_entry::DirectoryEntry> = if is_root {
                Some(directory_entry::DirectoryEntry::volume_label("THEOS"))
            } else {
                None
            };
            match volume_label {
                Some(volume_label) => directory_entries.push(volume_label),
                None => (),
            }
            let volume_guid: Option<directory_entry::DirectoryEntry> = if is_root && has_volume_guid {
                Some(directory_entry::DirectoryEntry::volume_guid(guid::Guid::new(rand_generator).to_u128()))
            } else {
                None
            };
            match volume_guid {
                Some(volume_guid) => directory_entries.push(volume_guid),
                None => (),
            }
            let allocation_bitmaps: Vec<directory_entry::DirectoryEntry> = if is_root {
                let volume_size_lower_limit: usize  = 1 << 20;
                let cluster_heap_offset: usize = (boot_sector.fat_offset() as usize + boot_sector.num_of_fats() as usize) * boot_sector.bytes_per_sector();
                let cluster_heap_offset: usize = (cluster_heap_offset + boot_sector.cluster_size() - 1) / boot_sector.cluster_size();
                let cluster_heap_offset: usize = cluster_heap_offset * boot_sector.cluster_size();
                let reserved_clusters: usize = 1/* root directory */ + boot_sector.num_of_fats()/* allocation bitmap */;
                let reserved_clusters_size: usize = reserved_clusters * boot_sector.cluster_size();
                let reserved_size: usize = cluster_heap_offset + reserved_clusters_size;
                let fixed_clusters_size: usize = volume_size_lower_limit - reserved_size;
                clusters.fix_size(fixed_clusters_size);
                directory_entry::DirectoryEntry::allocation_bitmaps(clusters, &directory_entries, boot_sector.num_of_fats())
            } else {
                vec![]
            };
            let mut allocation_bitmaps: Vec<directory_entry::DirectoryEntry> = allocation_bitmaps
                .into_iter()
                .collect();
            directory_entries.append(&mut allocation_bitmaps);
            let bytes: Vec<u8> = directory_entries
                .iter()
                .map(|directory_entry| Into::<Vec<u8>>::into(directory_entry))
                .flatten()
                .collect();
            let length: usize = bytes.len();
            let first_cluster: u32 = clusters.append(&bytes, 0);
            let directory = Self::Directory {
                children,
                directory_entries,
            };
            (directory, first_cluster, length)
        } else {
            panic!("Can't find {}!", source.display());
        }
    }

    pub fn read_directory(
        destination: &PathBuf,
        clusters: &cluster::Clusters,
        fat: &fat::Fat,
        cluster_number: u32,
        cluster_size: usize
    ) -> Self {
        let directory_entries: Vec<u8> = clusters.cluster_chain_bytes(cluster_number);
        let directory_entries: Vec<directory_entry::DirectoryEntry> = directory_entry::DirectoryEntry::read(&directory_entries, clusters);
        let file_directory_entries: Vec<Option<directory_entry::DirectoryEntry>> = directory_entries
            .iter()
            .filter(|directory_entry| match directory_entry {
                directory_entry::DirectoryEntry::File {
                    file_attributes: _,
                    create_time: _,
                    modified_time: _,
                    accessed_time: _,
                    stream_extension: _,
                } => true,
                _ => false,
            })
            .map(|directory_entry| Some(directory_entry.clone()))
            .collect();
        let children: Vec<Rc<Node>> = file_directory_entries
            .iter()
            .map(|file_directory_entry| Node::read(destination, file_directory_entry, clusters, fat, cluster_size))
            .collect();
        let children: RefCell<Vec<Rc<Node>>> = RefCell::new(children);
        Self::Directory {
            children,
            directory_entries,
        }
    }

    pub fn read_file(clusters: &cluster::Clusters, cluster_number: u32, length: usize) -> Self {
        let mut bytes: Vec<u8> = clusters.cluster_chain_bytes(cluster_number);
        bytes.resize(length, 0x00);
        Self::File {
            bytes,
        }
    }

    pub fn upcase_table(&self) -> upcase_table::UpcaseTable {
        if let Self::Directory {
            children: _,
            directory_entries,
        } = self {
            directory_entries
                .iter()
                .find_map(|directory_entry| if let directory_entry::DirectoryEntry::UpcaseTable {
                    table_checksum: _,
                    first_cluster: _,
                    data_length: _,
                    upcase_table,
                } = directory_entry {
                    Some(upcase_table.clone())
                } else {
                    None
                })
                .expect("Can't get an upcase table.")
        } else {
            panic!("Can't get an upcase table.")
        }
    }

    pub fn volume_guid(&self) -> Option<guid::Guid> {
        if let Self::Directory {
            children: _,
            directory_entries,
        } = self {
            directory_entries
                .iter()
                .find_map(|directory_entry| if let directory_entry::DirectoryEntry::VolumeGuid {
                    general_flags: _,
                    volume_guid,
                } = directory_entry {
                    Some(guid::Guid::read(*volume_guid))
                } else {
                    None
                })
        } else {
            None
        }
    }

    pub fn volume_label(&self) -> String {
        if let Self::Directory {
            children: _,
            directory_entries,
        } = self {
            directory_entries
                .iter()
                .find_map(|directory_entry| if let directory_entry::DirectoryEntry::VolumeLabel {
                    volume_label,
                } = directory_entry {
                    Some(volume_label.clone())
                } else {
                    None
                })
                .expect("Can't get a volume label.")
        } else {
            panic!("Can't get a volume label.")
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
                directory_entries: _,
            } => children
                .borrow()
                .iter()
                .map(|child| format!("{}", child))
                .fold(String::new(), |string, child| string + &child),
        };
        write!(f, "{}", string)
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    content: Content,
    destination: PathBuf,
    directory_entry: Option<directory_entry::DirectoryEntry>,
    first_cluster: u32,
    parent: RefCell<Weak<Self>>,
}

impl Node {
    pub fn allocation_bitmap(&self, clusters: &cluster::Clusters) -> allocation_bitmap::AllocationBitmap {
        self.content.allocation_bitmap(clusters)
    }

    pub fn first_cluster(&self) -> u32 {
        self.first_cluster
    }

    pub fn read_root_directory(
        clusters: &cluster::Clusters,
        fat: &fat::Fat,
        first_cluster: u32,
        cluster_size: usize
    ) -> Rc<Self> {
        let destination = PathBuf::from("/");
        let content = Content::read_directory(&destination, clusters, fat, first_cluster, cluster_size);
        let directory_entry: Option<directory_entry::DirectoryEntry> = None;
        let parent = RefCell::new(Weak::new());
        let object = Rc::new(Self {
            content,
            destination,
            directory_entry,
            first_cluster,
            parent,
        });
        if let Content::Directory{
            children,
            directory_entries: _,
        } = &object.content {
            children
                .borrow_mut()
                .iter_mut()
                .for_each(|child| *child.parent.borrow_mut() = Rc::downgrade(&object));
        }
        object
    }

    pub fn root_directory(
        source: &PathBuf,
        boot_sector: &boot_sector::BootSector,
        clusters: &mut cluster::Clusters,
        upcase_table: &upcase_table::UpcaseTable,
        has_volume_guid: bool,
        rand_generator: &mut rand::Generator,
    ) -> Rc<Self> {
        let destination = &PathBuf::from("/");
        let is_root: bool = true;
        Self::new(source, destination, is_root, boot_sector, clusters, upcase_table, has_volume_guid, rand_generator)
    }

    pub fn upcase_table(&self) -> upcase_table::UpcaseTable {
        match self.parent.borrow().upgrade() {
            Some(parent) => parent.upcase_table(),
            None => self.content.upcase_table(),
        }
    }

    pub fn volume_guid(&self) -> Option<guid::Guid> {
        self.content.volume_guid()
    }

    pub fn volume_label(&self) -> String {
        self.content.volume_label()
    }

    fn new(
        source: &PathBuf,
        destination: &PathBuf,
        is_root: bool,
        boot_sector: &boot_sector::BootSector,
        clusters: &mut cluster::Clusters,
        upcase_table: &upcase_table::UpcaseTable,
        has_volume_guid: bool,
        rand_generator: &mut rand::Generator,
    ) -> Rc<Self> {
        let (content, first_cluster, length) = Content::new(&source, &destination, is_root, boot_sector, clusters, upcase_table, has_volume_guid, rand_generator);
        let destination: PathBuf = destination.to_path_buf();
        let directory_entry = if is_root {
            None
        } else {
            Some(directory_entry::DirectoryEntry::file(&source, first_cluster, length, clusters.cluster_size(), upcase_table))
        };
        let parent = RefCell::new(Weak::new());
        let object = Rc::new(Self {
            content,
            destination,
            directory_entry,
            first_cluster,
            parent,
        });
        if let Content::Directory {
            children,
            directory_entries: _,
        } = &object.content {
            children
                .borrow_mut()
                .iter_mut()
                .for_each(|child | *child.parent.borrow_mut() = Rc::downgrade(&object));
        }
        object
    }

    fn read(parent: &PathBuf, directory_entry: &Option<directory_entry::DirectoryEntry>, clusters: &cluster::Clusters, fat: &fat::Fat, cluster_size: usize) -> Rc<Self> {
        let directory_entry: Option<directory_entry::DirectoryEntry> = directory_entry.clone();
        if let Some(directory_entry::DirectoryEntry::File {
            ref file_attributes,
            create_time: _,
            modified_time: _,
            accessed_time: _,
            ref stream_extension,
        }) = directory_entry {
            if let directory_entry::DirectoryEntry::StreamExtension {
                general_flags: _,
                name_length: _,
                name_hash: _,
                first_cluster,
                data_length,
                file_name: _,
            } = &**stream_extension {
                let file_name: String = match directory_entry {
                    Some(ref directory_entry) => directory_entry
                        .get_file_name()
                        .expect("Can't read an object."),
                    None => "".to_string(),
                };
                let mut destination: PathBuf = parent.to_path_buf();
                destination.push(file_name);
                let first_cluster: u32 = *first_cluster;
                let content = if file_attributes.is_dir() {
                    Content::read_directory(&destination, clusters, fat, first_cluster, cluster_size)
                } else {
                    Content::read_file(clusters, first_cluster, *data_length)
                };
                let parent = RefCell::new(Weak::new());
                let object = Rc::new(Self {
                    content,
                    destination,
                    directory_entry,
                    first_cluster,
                    parent,
                });
                if let Content::Directory{
                    children,
                    directory_entries: _,
                } = &object.content {
                    children
                        .borrow_mut()
                        .iter_mut() 
                        .for_each(|child| *child.parent.borrow_mut() = Rc::downgrade(&object));
                }
                object
            } else {
                panic!("Can't read an object.");
            }
        } else {
            panic!("Can't read an object.");
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path: String = format!("{}", self.destination.display());
        let path: String = self
            .upcase_table()
            .capitalize_str(&path);
        let content: String = format!("{}", self.content);
        write!(f, "{}\n{}", path, content)
    }
}

