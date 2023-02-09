mod attribute;

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
        attribute: attribute::Attribute,
        accessed_time: time::Time,
        created_time: time::Time,
        written_time: time::Time,
        first_cluster: u32
    },
}

impl From<&node::Node> for DirectoryEntry {
    fn from(node: &node::Node) -> Self {
        let attribute = attribute::Attribute::from(node);
        let accessed_time: time::Time = node.last_accessed_time();
        let created_time: time::Time = node.last_changed_time();
        let written_time: time::Time = node.last_modified_time();
        let first_cluster: u32 = node.first_cluster();
        Self::ShortFileName {
            attribute,
            accessed_time,
            created_time,
            written_time,
            first_cluster,
        }
    }
}

