use std::env;

fn main() {
	let project_directory: &str = "CARGO_MANIFEST_DIR";
	let project_directory: String = env::var(project_directory).expect(&format!("Can't get {}", project_directory));
	println!("cargo:rustc-link-search={}/ffi/", project_directory);
	println!("cargo:rustc-link-lib=stat");
}

