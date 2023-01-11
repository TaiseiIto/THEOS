use std::path;

#[derive(Debug)]
pub struct Object {
	path: path::PathBuf,
}

impl Object {
	pub fn new(path: path::PathBuf) -> Self {
		Self {
			path,
		}
	}
}

