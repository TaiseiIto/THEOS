use std::path;

#[derive(Debug)]
pub enum DirectoryEntry {
	File {
		file_attributes: FileAttributes,
	},
}

impl DirectoryEntry {
	pub fn file(path: &path::PathBuf) -> Self {
		let file_attributes = FileAttributes::new(path);
		Self::File {
			file_attributes,
		}
	}

	fn entry_type(&self) -> EntryType {
		EntryType::file()
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
	pub fn file() -> Self {
		let type_code = TypeCode::file();
		let type_importance = false;
		let type_category = false;
		let in_use = true;
		Self {
			type_code,
			type_importance,
			type_category,
			in_use,
		}
	}
}

#[derive(Debug)]
enum TypeCode {
	File,
}

impl TypeCode {
	pub fn file() -> Self {
		Self::File
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
	fn new(path: &path::PathBuf) -> Self {
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

