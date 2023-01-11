use std::{
	fs,
	path,
};

#[derive(Debug)]
pub struct Object {
	path: path::PathBuf,
	content: FileOrDirectory,
}

impl Object {
	pub fn new(path: path::PathBuf) -> Self {
		let content = FileOrDirectory::new(&path);
		Self {
			path,
			content,
		}
	}
}

#[derive(Debug)]
enum FileOrDirectory {
	File {
		bytes: Vec<u8>,
	},
	Directory {
		children: Vec<Object>,
	},
}

impl FileOrDirectory {
	fn new(path: &path::PathBuf) -> Self {
		if path.is_file() {
			let bytes: Vec<u8> = fs::read(path).expect(&format!("Can't read {}!", path.display()));
			Self::File {
				bytes,
			}
		} else if path.is_dir() {
			let children: Vec<Object> = match fs::read_dir(path) {
				Ok(directory) => directory
					.into_iter()
					.filter_map(|directory| directory.ok())
					.map(|directory| Object::new(directory.path()))
					.collect(),
				_ => vec![],
			};
			Self::Directory {
				children,
			}
		} else {
			panic!("Can't find {}!", path.display());
		}
	}
}

