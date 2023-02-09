mod attribute;
mod short_file_name;

use {
    std::convert::From,
    super::{
        node,
        super::super::time,
    },
};

#[derive(Debug)]
pub enum DirectoryEntry {
    ShortFileName {
        name: String,
        attribute: attribute::Attribute,
        accessed_time: time::Time,
        created_time: time::Time,
        written_time: time::Time,
        first_cluster: u32,
        size: usize,
        long_file_name: Option<Box<Self>>,
    },
    LongFileName {
        name: [u16; LONG_FILE_NAME_LENGTH],
        order: usize,
        next: Option<Box<Self>>,
    },
}

const LONG_FILE_NAME_LENGTH: usize = 13;

impl DirectoryEntry {
    fn long_file_name(mut name: Vec<u16>, order: usize) -> Self {
        let (name, next): ([u16; LONG_FILE_NAME_LENGTH], Option<Box<Self>>) = if LONG_FILE_NAME_LENGTH <= name.len() {
            let (name, next): (&[u16], &[u16]) = name.split_at(LONG_FILE_NAME_LENGTH);
            let name: [u16; LONG_FILE_NAME_LENGTH] = name
                .try_into()
                .expect("Can't generate a long file name directory entry.");
            let next: Vec<u16> = next.to_vec();
            let next: Option<Box<Self>> = Some(Box::new(Self::long_file_name(next, order + 1)));
            (name, next)
        } else {
            name.resize(LONG_FILE_NAME_LENGTH, 0x0000);
            let name: [u16; LONG_FILE_NAME_LENGTH] = name
                .try_into()
                .expect("Can't generate a long file name directory entry.");
            let next: Option<Box<Self>> = None;
            (name, next)
        };
        Self::LongFileName {
            name,
            order,
            next,
        }
    }
}

impl From<&node::Node> for DirectoryEntry {
    fn from(node: &node::Node) -> Self {
        let name: String = node.name();
        let attribute = attribute::Attribute::from(node);
        let accessed_time: time::Time = node.last_accessed_time();
        let created_time: time::Time = node.last_changed_time();
        let written_time: time::Time = node.last_modified_time();
        let first_cluster: u32 = node.first_cluster();
        let size: usize = if node.is_directory() {
            0
        } else {
            node.size()
        };
        let long_file_name: Vec<u16> = name.encode_utf16().collect();
        let long_file_name: Option<Box<Self>> = Some(Box::new(Self::long_file_name(long_file_name, 1)));
        Self::ShortFileName {
            name,
            attribute,
            accessed_time,
            created_time,
            written_time,
            first_cluster,
            size,
            long_file_name,
        }
    }
}

