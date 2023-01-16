use {
    std::{
        fs,
        path,
    },
    super::{
        cluster,
        directory_entry,
        upcase_table,
    },
};

#[derive(Debug)]
pub struct Object {
    path: path::PathBuf,
    content: FileOrDirectory,
    directory_entry: directory_entry::DirectoryEntry,
}

impl Object {
    pub fn root(path: path::PathBuf, clusters: &mut cluster::Clusters, upcase_table: &upcase_table::UpcaseTable) -> Self {
        let is_root = true;
        Self::new(path, is_root, clusters, upcase_table)
    }

    fn new(path: path::PathBuf, is_root: bool, clusters: &mut cluster::Clusters, upcase_table: &upcase_table::UpcaseTable) -> Self {
        let content = FileOrDirectory::new(&path, is_root, clusters, upcase_table);
        let directory_entry = directory_entry::DirectoryEntry::file(&path, &content, upcase_table);
        Self {
            path,
            content,
            directory_entry,
        }
    }
}

#[derive(Debug)]
pub enum FileOrDirectory {
    File {
        first_cluster: u32,
        length: usize,
    },
    Directory {
        children: Vec<Object>,
        first_cluster: u32,
        length: usize,
    },
}

impl FileOrDirectory {
    fn new(path: &path::PathBuf, is_root: bool, clusters: &mut cluster::Clusters, upcase_table: &upcase_table::UpcaseTable) -> Self {
        if path.is_file() {
            let mut bytes: Vec<u8> = fs::read(path).expect(&format!("Can't read {}!", path.display()));
            let length: usize = bytes.len();
            let first_cluster: u32 = clusters.append(bytes);
            Self::File {
                first_cluster,
                length,
            }
        } else if path.is_dir() {
            let children: Vec<Object> = match fs::read_dir(path) {
                Ok(directory) => directory
                    .into_iter()
                    .filter_map(|directory| directory.ok())
                    .map(|directory| Object::new(directory.path(), false, clusters, upcase_table))
                    .collect(),
                _ => vec![],
            };
            let bytes: Vec<u8> = children
                .iter()
                .map(|object| object.directory_entry.entry_set_to_bytes())
                .flatten()
                .collect();
            let length: usize = bytes.len();
            let first_cluster: u32 = clusters.append(bytes);
            Self::Directory {
                children,
                first_cluster,
                length,
            }
        } else {
            panic!("Can't find {}!", path.display());
        }
    }
}

