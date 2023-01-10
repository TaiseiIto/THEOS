extern crate regex;

use std::fmt;
use super::{
    object,
    super::time,
};

#[derive(Debug)]
pub struct DirectoryEntry {
    entry_type: EntryType,
    inner: DirectoryEntryEnum,
}

impl DirectoryEntry {
    pub fn file_directory(
        object: &object::FileOrDirectory,
        create_time: &time::Time,
        modified_time: &time::Time,
        accessed_time: &time::Time,
    ) -> Self {
        Self {
            entry_type: EntryType::file_directory(object),
            inner: DirectoryEntryEnum::file_directory(
                object,
                create_time,
                modified_time,
                accessed_time,
            ),
        }
    }
}

impl fmt::Display for DirectoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let regex = regex::Regex::new("^|\n").expect("Can't create a Regex.");
        let entry_type: String = format!("{}", self.entry_type);
        let entry_type: String = regex.replace_all(&entry_type, "$0directory_entry.");
        write!(f, "{}\n", entry_type)?;
        let inner: String = format!("{}", self.inner);
        let inner: String = regex.replace_all(&inner, "$0directory_entry.");
        write!(f, "{}", inner)
    }
}

#[derive(Debug)]
struct EntryType {
    type_code: TypeCode,
    type_importance: bool,
    type_category: bool,
    in_use: bool,
}

impl EntryType {
    fn file_directory(object: &object::FileOrDirectory) -> Self {
        Self {
            type_code: TypeCode::FileDirectory,
            type_importance: false,
            type_category: false,
            in_use: true,
        }
    }
}

impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "entry_type.type_code = {}\n", self.type_code)?;
        write!(f, "entry_type.type_importance = {}\n", self.type_importance)?;
        write!(f, "entry_type.type_category = {}\n", self.type_category)?;
        write!(f, "entry_type.in_use = {}", self.in_use)
    }
}

#[derive(Debug)]
enum TypeCode {
    AllocationBitmap,
    UpcaseTable,
    VolumeLabel,
    FileDirectory,
    StreamExtension,
    FileName,
}

impl fmt::Display for TypeCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AllocationBitmap => write!(f, "AllocationBitmap"),
            Self::UpcaseTable => write!(f, "UpcaseTable"),
            Self::VolumeLabel => write!(f, "VolumeLabel"),
            Self::FileDirectory => write!(f, "FileDirectory"),
            Self::StreamExtension => write!(f, "StreamExtension"),
            Self::FileName => write!(f, "FileName"),
        }
    }
}

#[derive(Debug)]
enum DirectoryEntryEnum {
    AllocationBitmap,
    UpcaseTable,
    VolumeLabel,
    FileDirectory {
        secondary_count: u8,
        set_checksum: u16,
        file_attributes: FileAttributes,
        create_time: time::Time,
        modified_time: time::Time,
        accessed_time: time::Time,
    },
    StreamExtension {
		allocation_possible: bool,
		no_fat_chain: bool,
		name_length: u8,
		name_hash: u16,
		first_cluster: u32,
		data_length: u64,
	},
    FileName,
}

impl DirectoryEntryEnum {
    fn file_directory(
        object: &object::FileOrDirectory,
        create_time: &time::Time,
        modified_time: &time::Time,
        accessed_time: &time::Time,
    ) -> Self {
        Self::FileDirectory {
            secondary_count: 0,
            set_checksum: 0,
            file_attributes: FileAttributes::file_directory(object),
            create_time: *create_time,
            modified_time: *modified_time,
            accessed_time: *accessed_time,
        }
    }
}

impl fmt::Display for DirectoryEntryEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let regex = regex::Regex::new("^|\n").expect("Can't create a Regex.");
        match self {
            Self::AllocationBitmap => write!(f, "AllocationBitmap"),
            Self::UpcaseTable => write!(f, "UpcaseTable"),
            Self::VolumeLabel => write!(f, "VolumeLabel"),
            Self::FileDirectory {
                secondary_count,
                set_checksum,
                file_attributes,
                create_time,
                modified_time,
                accessed_time,
            } => {
                write!(f, "FileDirectory.secondary_count = {}\n", secondary_count)?;
                write!(f, "FileDirectory.set_checksum = {}\n", set_checksum)?;
                let file_attributes: String = format!("{}", file_attributes);
                let file_attributes: String = regex.replace_all(&file_attributes, "$0FileDirectory.");
                write!(f, "{}\n", file_attributes)?;
                let create_time: String = format!("{}", create_time);
                let create_time: String = regex.replace_all(&create_time, "$0create_time.");
                write!(f, "{}\n", create_time)?;
                let modified_time: String = format!("{}", modified_time);
                let modified_time: String = regex.replace_all(&modified_time, "$0modified_time.");
                write!(f, "{}\n", modified_time)?;
                let accessed_time: String = format!("{}", accessed_time);
                let accessed_time: String = regex.replace_all(&accessed_time, "$0accessed_time.");
                write!(f, "{}", accessed_time)
            },
            Self::StreamExtension {
				allocation_possible,
				no_fat_chain,
				name_length,
				name_hash,
				first_cluster,
				data_length,
			} => {
				write!(f, "StreamExtension.allocation_possible = {}\n", allocation_possible)?;
				write!(f, "StreamExtension.no_fat_chain = {}\n", no_fat_chain)?;
				write!(f, "StreamExtension.name_length = {}\n", name_length)?;
				write!(f, "StreamExtension.name_hash = {}\n", name_hash)?;
				write!(f, "StreamExtension.first_cluster = {}\n", first_cluster)?;
				write!(f, "StreamExtension.data_length = {}", data_length)
			}
            Self::FileName => write!(f, "FileName"),
        }
    }
}

#[derive(Debug)]
struct FileAttributes {
    read_only: bool,
    hidden: bool,
    system: bool,
    directory: bool,
    archive: bool,
}

impl FileAttributes {
    fn file_directory(object: &object::FileOrDirectory) -> Self {
        Self {
            read_only: true,
            hidden: false,
            system: true,
            directory: match object {
                object::FileOrDirectory::File {
                    bytes,
                } => false,
                object::FileOrDirectory::Directory {
                    children,
                } => true,
            },
            archive: false,
        }
    }
}

impl fmt::Display for FileAttributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "file_attributes.read_only = {}\n", self.read_only)?;
        write!(f, "file_attributes.hidden = {}\n", self.hidden)?;
        write!(f, "file_attributes.system = {}\n", self.system)?;
        write!(f, "file_attributes.directory = {}\n", self.directory)?;
        write!(f, "file_attributes.archive = {}", self.archive)
    }
}

