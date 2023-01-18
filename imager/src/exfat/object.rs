use {
    std::{
        fs,
        path,
    },
    super::{
        allocation_bitmap,
        boot_sector,
        cluster,
        directory_entry,
        upcase_table,
    },
};

#[derive(Debug)]
pub struct Object {
    path: path::PathBuf,
    content: FileOrDirectory,
    first_cluster: u32,
    length: usize,
    directory_entry: directory_entry::DirectoryEntry,
}

impl Object {
    pub fn root(path: path::PathBuf, boot_sector: &boot_sector::BootSector, clusters: &mut cluster::Clusters, upcase_table: &upcase_table::UpcaseTable) -> Self {
        Self::new(path, true, boot_sector, clusters, upcase_table)
    }

    fn new(path: path::PathBuf, is_root: bool, boot_sector: &boot_sector::BootSector, clusters: &mut cluster::Clusters, upcase_table: &upcase_table::UpcaseTable) -> Self {
        let (content, first_cluster, length) = FileOrDirectory::new(&path, is_root, boot_sector, clusters, upcase_table);
        let directory_entry = directory_entry::DirectoryEntry::file(&path, first_cluster, length, upcase_table);
        Self {
            path,
            content,
            first_cluster,
            length,
            directory_entry,
        }
    }
}

#[derive(Debug)]
pub enum FileOrDirectory {
    File,
    Directory {
        children: Vec<Object>,
    },
}

impl FileOrDirectory {
    fn new(path: &path::PathBuf, is_root: bool, boot_sector: &boot_sector::BootSector, clusters: &mut cluster::Clusters, upcase_table: &upcase_table::UpcaseTable) -> (Self, u32, usize) {
        if path.is_file() {
            let mut bytes: Vec<u8> = fs::read(path).expect(&format!("Can't read {}!", path.display()));
            let length: usize = bytes.len();
            let first_cluster: u32 = clusters.append(bytes, 0);
            let file = Self::File;
            (file, first_cluster, length)
        } else if path.is_dir() {
            let children: Vec<Object> = match fs::read_dir(path) {
                Ok(directory) => directory
                    .into_iter()
                    .filter_map(|directory| directory.ok())
                    .map(|directory| Object::new(directory.path(), false, boot_sector, clusters, upcase_table))
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
                .map(|directory_entry| directory_entry.entry_set_to_bytes())
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
}

