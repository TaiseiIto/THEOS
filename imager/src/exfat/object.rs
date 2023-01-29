use {
    std::{
        fs,
        mem,
        path,
    },
    super::{
        allocation_bitmap,
        boot_sector,
        cluster,
        directory_entry,
        fat,
        super::{
            binary::Binary,
            guid,
            rand,
        },
        upcase_table,
    },
};

#[derive(Clone, Debug)]
pub enum FileOrDirectory {
    File {
        bytes: Vec<u8>,
    },
    Directory {
        children: Vec<Object>,
        directory_entries: Vec<directory_entry::DirectoryEntry>,
    },
}

impl FileOrDirectory {
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
        source_path: &path::PathBuf,
        is_root: bool,
        boot_sector: &boot_sector::BootSector,
        clusters: &mut cluster::Clusters,
        upcase_table: &upcase_table::UpcaseTable,
        rand_generator: &mut rand::Generator,
    ) -> (Self, u32, usize) {
        if source_path.is_file() {
            let bytes: Vec<u8> = fs::read(source_path).expect(&format!("Can't read {}!", source_path.display()));
            let length: usize = bytes.len();
            let first_cluster: u32 = clusters.append(&bytes, 0);
            let file = Self::File {
                bytes,
            };
            (file, first_cluster, length)
        } else if source_path.is_dir() {
            let children: Vec<Object> = match fs::read_dir(source_path) {
                Ok(directory) => directory
                    .into_iter()
                    .filter_map(|directory| directory.ok())
                    .map(|directory| Object::new(directory.path(), false, boot_sector, clusters, upcase_table, rand_generator))
                    .collect(),
                _ => vec![],
            };
            let mut directory_entries: Vec<directory_entry::DirectoryEntry> = children
                .iter()
                .map(|object| object.directory_entry.clone())
                .collect();
            let upcase_table: Option<directory_entry::DirectoryEntry> = match is_root {
                true => Some(directory_entry::DirectoryEntry::upcase_table(upcase_table, clusters)),
                false => None,
            };
            match upcase_table {
                Some(upcase_table) => directory_entries.push(upcase_table),
                None => (),
            }
            let volume_label: Option<directory_entry::DirectoryEntry> = match is_root {
                true => Some(directory_entry::DirectoryEntry::volume_label("THEOS")),
                false => None,
            };
            match volume_label {
                Some(volume_label) => directory_entries.push(volume_label),
                None => (),
            }
            let volume_guid: Option<directory_entry::DirectoryEntry> = match is_root {
                true => Some(directory_entry::DirectoryEntry::volume_guid(guid::Guid::new(rand_generator).to_u128())),
                false => None,
            };
            match volume_guid {
                Some(volume_guid) => directory_entries.push(volume_guid),
                None => (),
            }
            let allocation_bitmaps: Vec<directory_entry::DirectoryEntry> = match is_root {
                true => directory_entry::DirectoryEntry::allocation_bitmaps(clusters, &directory_entries, boot_sector.num_of_fats()),
                false => vec![],
            };
            let mut allocation_bitmaps: Vec<directory_entry::DirectoryEntry> = allocation_bitmaps
                .into_iter()
                .collect();
            directory_entries.append(&mut allocation_bitmaps);
            let bytes: Vec<u8> = directory_entries
                .iter()
                .map(|directory_entry| directory_entry.to_bytes())
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
            panic!("Can't find {}!", source_path.display());
        }
    }

    pub fn read_directory(clusters: &cluster::Clusters, fat: &fat::Fat, cluster_number: u32, cluster_size: usize) -> Self {
        let directory_entries: Vec<u8> = clusters.get_bytes(cluster_number);
        let directory_entries: Vec<directory_entry::DirectoryEntry> = directory_entry::DirectoryEntry::read(&directory_entries);
        let file_directory_entries: Vec<directory_entry::DirectoryEntry> = directory_entries
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
            .map(|directory_entry| directory_entry.clone())
            .collect();
        let children: Vec<Object> = file_directory_entries
            .iter()
            .map(|file_directory_entry| Object::read(file_directory_entry, clusters, fat, cluster_size))
            .collect();
        Self::Directory {
            children,
            directory_entries,
        }
    }

    pub fn read_file(clusters: &cluster::Clusters, fat: &fat::Fat, cluster_number: u32, cluster_size: usize, length: usize) -> Self {
        let mut bytes: Vec<u8> = clusters.get_bytes(cluster_number);
        bytes.resize(length, 0x00);
        Self::File {
            bytes,
        }
    }

    pub fn upcase_table(&self, clusters: &cluster::Clusters) -> upcase_table::UpcaseTable {
        if let Self::Directory {
            children: _,
            directory_entries,
        } = self {
            directory_entries
                .iter()
                .find_map(|directory_entry| if let directory_entry::DirectoryEntry::UpcaseTable {
                    table_checksum: _,
                    first_cluster,
                    data_length,
                } = directory_entry {
                    Some(clusters.upcase_table(*first_cluster, *data_length))
                } else {
                    None
                })
                .expect("Can't get an upcase table.")
        } else {
            panic!("Can't get an upcase table.")
        }
    }

    pub fn volume_guid(&self) -> guid::Guid {
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
                .expect("Can't get a volume GUID.")
        } else {
            panic!("Can't get a volume GUID.")
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

#[derive(Clone, Debug)]
pub struct Object {
    content: FileOrDirectory,
    directory_entry: directory_entry::DirectoryEntry,
    first_cluster: u32,
}

impl Object {
    pub fn content(&self) -> FileOrDirectory {
        self.content.clone()
    }

    pub fn first_cluster(&self) -> u32 {
        self.first_cluster
    }

    pub fn root(
        source_path: path::PathBuf,
        boot_sector: &boot_sector::BootSector,
        clusters: &mut cluster::Clusters,
        upcase_table: &upcase_table::UpcaseTable,
        rand_generator: &mut rand::Generator,
    ) -> Self {
        Self::new(source_path, true, boot_sector, clusters, upcase_table, rand_generator)
    }

    fn new(
        source_path: path::PathBuf,
        is_root: bool,
        boot_sector: &boot_sector::BootSector,
        clusters: &mut cluster::Clusters,
        upcase_table: &upcase_table::UpcaseTable,
        rand_generator: &mut rand::Generator,
    ) -> Self {
        let (content, first_cluster, length) = FileOrDirectory::new(&source_path, is_root, boot_sector, clusters, upcase_table, rand_generator);
        let directory_entry = directory_entry::DirectoryEntry::file(&source_path, first_cluster, length, upcase_table);
        Self {
            content,
            directory_entry,
            first_cluster,
        }
    }

    fn read(directory_entry: &directory_entry::DirectoryEntry, clusters: &cluster::Clusters, fat: &fat::Fat, cluster_size: usize) -> Self {
        let directory_entry: directory_entry::DirectoryEntry = directory_entry.clone();
        if let directory_entry::DirectoryEntry::File {
            ref file_attributes,
            create_time: _,
            modified_time: _,
            accessed_time: _,
            ref stream_extension,
        } = directory_entry {
            if let directory_entry::DirectoryEntry::StreamExtension {
                general_flags: GeneralFlags,
                name_length: _,
                name_hash: _,
                first_cluster,
                data_length,
                file_name: _,
            } = &**stream_extension {
                let first_cluster: u32 = *first_cluster;
                let content = if file_attributes.is_dir() {
                    FileOrDirectory::read_directory(clusters, fat, first_cluster, cluster_size)
                } else {
                    FileOrDirectory::read_file(clusters, fat, first_cluster, cluster_size, *data_length)
                };
                Self {
                    content,
                    directory_entry,
                    first_cluster,
                }
            } else {
                panic!("Can't read an object.");
            }
        } else {
            panic!("Can't read an object.");
        }
    }
}

