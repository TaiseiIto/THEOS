use {
    std::convert::{
        From,
        Into,
    },
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

impl Attribute {
    pub fn long_file_name() -> Self {
        Self {
            read_only: false,
            hidden: false,
            system: false,
            volume_id: false,
            directory: false,
            archive: false,
            long_file_name: true,
        }
    }
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

impl From<u8> for Attribute {
    fn from(byte: u8) -> Self {
        let read_only: bool = byte & READ_ONLY != 0;
        let hidden: bool = byte & HIDDEN != 0;
        let system: bool = byte & SYSTEM != 0;
        let volume_id: bool = byte & VOLUME_ID != 0;
        let directory: bool = byte & DIRECTORY != 0;
        let archive: bool = byte & ARCHIVE != 0;
        let long_file_name: bool = byte == LONG_FILE_NAME;
        if long_file_name {
            let read_only: bool = false;
            let hidden: bool = false;
            let system: bool = false;
            let volume_id: bool = false;
            let directory: bool = false;
            let archive: bool = false;
            Self {
                read_only,
                hidden,
                system,
                volume_id,
                directory,
                archive,
                long_file_name,
            }
        } else {
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
}

impl Into<u8> for &Attribute {
    fn into(self) -> u8 {
        let read_only: u8 = if self.read_only {
            READ_ONLY
        } else {
            0
        };
        let hidden: u8 = if self.hidden {
           HIDDEN
        } else {
            0
        };
        let system: u8 = if self.system {
            SYSTEM
        } else {
            0
        };
        let volume_id: u8 = if self.volume_id {
            VOLUME_ID
        } else {
            0
        };
        let directory: u8 = if self.directory {
            DIRECTORY
        } else {
            0
        };
        let archive: u8 = if self.archive {
            ARCHIVE
        } else {
            0
        };
        if self.long_file_name {
            LONG_FILE_NAME
        } else {
            read_only + hidden + system + volume_id + directory + archive
        }
    }
}

const READ_ONLY: u8 = 0x01;
const HIDDEN: u8 = 0x02;
const SYSTEM: u8 = 0x04;
const VOLUME_ID: u8 = 0x08;
const DIRECTORY: u8 = 0x10;
const ARCHIVE: u8 = 0x20;
const LONG_FILE_NAME: u8 = 0xf;

