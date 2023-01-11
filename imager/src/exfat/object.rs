use {
	std::{
		fs,
		path,
	},
	super::cluster,
};

#[derive(Debug)]
pub struct Object {
	path: path::PathBuf,
	content: FileOrDirectory,
}

impl Object {
	pub fn new(path: path::PathBuf, clusters: &mut cluster::Clusters) -> Self {
		let content = FileOrDirectory::new(&path, clusters);
		Self {
			path,
			content,
		}
	}
}

#[derive(Debug)]
enum FileOrDirectory {
	File {
		first_cluster: u32,
		length: usize,
	},
	Directory {
		children: Vec<Object>,
	},
}

impl FileOrDirectory {
	fn new(path: &path::PathBuf, clusters: &mut cluster::Clusters) -> Self {
		if path.is_file() {
			let mut bytes: Vec<u8> = fs::read(path).expect(&format!("Can't read {}!", path.display()));
			let length = bytes.len();
			let first_cluster = clusters.append(bytes);
			Self::File {
				first_cluster,
				length,
			}
		} else if path.is_dir() {
			let children: Vec<Object> = match fs::read_dir(path) {
				Ok(directory) => directory
					.into_iter()
					.filter_map(|directory| directory.ok())
					.map(|directory| Object::new(directory.path(), clusters))
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

