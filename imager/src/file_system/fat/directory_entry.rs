mod attribute;

use {
    std::convert::From,
    super::node,
};

#[derive(Debug)]
pub enum DirectoryEntry {
    ShortFileName {
        attribute: attribute::Attribute,
    },
}

impl From<&node::Node> for DirectoryEntry {
    fn from(node: &node::Node) -> Self {
        let attribute = attribute::Attribute::from(node);
        Self::ShortFileName {
            attribute,
        }
    }
}

