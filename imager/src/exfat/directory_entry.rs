#[derive(Debug)]
pub struct DirectoryEntry {
	entry_type: EntryType,
}

impl DirectoryEntry {
	pub fn file() -> Self {
		let entry_type = EntryType::file();
		Self {
			entry_type,
		}
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

