use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct FileAttributes {
    read_only: bool,
    hidden: bool,
    system: bool,
    directory: bool,
    archive: bool,
}

impl FileAttributes {
    pub fn is_dir(&self) -> bool {
        self.directory
    }

    pub fn new(path: &PathBuf) -> Self {
        let read_only = true;
        let hidden = false;
        let system = true;
        let directory = path.is_dir();
        let archive = false;
        Self {
            read_only,
            hidden,
            system,
            directory,
            archive,
        }
    }
}

impl Into<u16> for &FileAttributes {
    fn into(self) -> u16 {
        let read_only: u16 = match self.read_only {
            true => 1,
            false => 0,
        };
        let hidden: u16 = match self.hidden {
            true => 1 << 1,
            false => 0,
        };
        let system: u16 = match self.system {
            true => 1 << 2,
            false => 0,
        };
        let directory: u16 = match self.directory {
            true => 1 << 4,
            false => 0,
        };
        let archive: u16 = match self.archive {
            true => 1 << 5,
            false => 0,
        };
        read_only + hidden + system + directory + archive
    }
}

impl From<u16> for FileAttributes {
    fn from(word: u16) -> Self {
        let read_only: bool = word & 0x0001 != 0;
        let hidden: bool = word & 0x0002 != 0;
        let system: bool = word & 0x0004 != 0;
        let directory: bool = word & 0x0010 != 0;
        let archive: bool = word & 0x0020 != 0;
        Self {
            read_only,
            hidden,
            system,
            directory,
            archive,
        }
    }
}

