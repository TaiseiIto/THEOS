use {
    std::{
        fs,
        path,
    },
    super::{
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

#[derive(Debug)]
pub enum FileOrDirectory {
    File,
    Directory {
        children: Vec<Object>,
    },
}

impl FileOrDirectory {
    fn new(
        path: &path::PathBuf,
        is_root: bool,
        boot_sector: &boot_sector::BootSector,
        clusters: &mut cluster::Clusters,
        upcase_table: &upcase_table::UpcaseTable,
        rand_generator: &mut rand::Generator,
    ) -> (Self, u32, usize) {
        if path.is_file() {
            let bytes: Vec<u8> = fs::read(path).expect(&format!("Can't read {}!", path.display()));
            let length: usize = bytes.len();
            let first_cluster: u32 = clusters.append(bytes, 0);
            let file = Self::File;
            (file, first_cluster, length)
        } else if path.is_dir() {
            let children: Vec<Object> = match fs::read_dir(path) {
                Ok(directory) => directory
                    .into_iter()
                    .filter_map(|directory| directory.ok())
                    .map(|directory| Object::new(directory.path(), false, boot_sector, clusters, upcase_table, rand_generator))
                    .collect(),
                _ => vec![],
            };
            let mut directory_entries: Vec<&directory_entry::DirectoryEntry> = children
                .iter()
                .map(|object| &object.directory_entry)
                .collect();
            let upcase_table: Option<directory_entry::DirectoryEntry> = match is_root {
                true => Some(directory_entry::DirectoryEntry::upcase_table(upcase_table, clusters)),
                false => None,
            };
            match upcase_table {
                Some(ref upcase_table) => directory_entries.push(upcase_table),
                None => (),
            }
            let volume_label: Option<directory_entry::DirectoryEntry> = match is_root {
                true => Some(directory_entry::DirectoryEntry::volume_label("THEOS")),
                false => None,
            };
            match volume_label {
                Some(ref volume_label) => directory_entries.push(volume_label),
                None => (),
            }
            let volume_guid: Option<directory_entry::DirectoryEntry> = match is_root {
                true => Some(directory_entry::DirectoryEntry::volume_guid(guid::Guid::new(rand_generator).to_u128())),
                false => None,
            };
            match volume_guid {
                Some(ref volume_guid) => directory_entries.push(volume_guid),
                None => (),
            }
            let allocation_bitmaps: Vec<directory_entry::DirectoryEntry> = match is_root {
                true => directory_entry::DirectoryEntry::allocation_bitmaps(clusters, &directory_entries, boot_sector.num_of_fats()),
                false => vec![],
            };
            let mut allocation_bitmaps: Vec<&directory_entry::DirectoryEntry> = allocation_bitmaps
                .iter()
                .collect();
            directory_entries.append(&mut allocation_bitmaps);
            let bytes: Vec<u8> = directory_entries
                .iter()
                .map(|directory_entry| directory_entry.to_bytes())
                .flatten()
                .collect();
            let length: usize = bytes.len();
            let first_cluster: u32 = clusters.append(bytes, 0);
            let directory = Self::Directory {
                children,
            };
            (directory, first_cluster, length)
        } else {
            panic!("Can't find {}!", path.display());
        }
    }

    pub fn read_directory(bytes: &Vec<u8>, fat: &fat::Fat, cluster_number: u32, cluster_size: usize) {
        let clusters: Vec<Vec<u8>> = bytes
            .chunks(cluster_size)
            .map(|cluster| cluster.to_vec())
            .collect();
        let mut cluster_number: Option<u32> = Some(cluster_number);
        let mut cluster_chain: Vec<u32> = vec![];
        while let Some(last_cluster_number) = cluster_number {
            cluster_chain.push(last_cluster_number);
            cluster_number = fat.next_cluster_number(last_cluster_number);
        }
        let directory_entries: Vec<u8> = cluster_chain
            .into_iter()
            .map(|cluster_number| clusters[(cluster_number - 2) as usize].clone())
            .flatten()
            .collect();
        let directory_entries: Vec<directory_entry::DirectoryEntry> = directory_entry::DirectoryEntry::read(&directory_entries);
        let file_directory_entries: Vec<directory_entry::DirectoryEntry> = directory_entries
            .into_iter()
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
            .collect();
        let objects: Vec<Object> = file_directory_entries
            .into_iter()
            .map(|file_directory_entry| Object::read(file_directory_entry))
            .collect();
        println!("{:#?}", objects);
    }
}

#[derive(Debug)]
pub struct Object {
    first_cluster: u32,
    directory_entry: directory_entry::DirectoryEntry,
}

impl Object {
    pub fn first_cluster(&self) -> u32 {
        self.first_cluster
    }

    pub fn root(
        path: path::PathBuf,
        boot_sector: &boot_sector::BootSector,
        clusters: &mut cluster::Clusters,
        upcase_table: &upcase_table::UpcaseTable,
        rand_generator: &mut rand::Generator,
    ) -> Self {
        Self::new(path, true, boot_sector, clusters, upcase_table, rand_generator)
    }

    fn new(
        path: path::PathBuf,
        is_root: bool,
        boot_sector: &boot_sector::BootSector,
        clusters: &mut cluster::Clusters,
        upcase_table: &upcase_table::UpcaseTable,
        rand_generator: &mut rand::Generator,
    ) -> Self {
        let (_, first_cluster, length) = FileOrDirectory::new(&path, is_root, boot_sector, clusters, upcase_table, rand_generator);
        let directory_entry = directory_entry::DirectoryEntry::file(&path, first_cluster, length, upcase_table);
        Self {
            first_cluster,
            directory_entry,
        }
    }

    fn read(directory_entry: directory_entry::DirectoryEntry) -> Self {
        if let directory_entry::DirectoryEntry::File {
            file_attributes: _,
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
                data_length: _,
                file_name: _,
            } = &**stream_extension {
                let first_cluster: u32 = *first_cluster;
                Self {
                    first_cluster,
                    directory_entry,
                }
            } else {
                panic!("Can't read an object.");
            }
        } else {
            panic!("Can't read an object.");
        }
    }
}

