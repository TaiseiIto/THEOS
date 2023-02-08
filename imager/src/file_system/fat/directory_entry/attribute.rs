use {
    std::convert::From,
    super::super::node,
};

#[derive(Debug)]
pub struct Attribute {
    read_only: bool,
    hidden: bool,
    system: bool,
    volume_id: bool,
    directory: bool,
    archive: bool,
    long_file_name: bool,
}

impl From<&node::Node> for Attribute {
    fn from(node: &node::Node) -> Self {
        let read_only: bool = node.is_read_only();
        let hidden: bool = node.is_hidden();
        let system: bool = node.is_system();
        let volume_id: bool = false;
        let directory: bool = node.is_directory();
        let archive: bool = false;
        let long_file_name: bool = false;
        Self {
            read_only,
            hidden,
            system,
            volume_id,
            directory,
            archive,
            long_file_name,
        }
    }
}

