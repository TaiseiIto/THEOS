use std::env;

fn main() {
    let prj_dir: &str = "CARGO_MANIFEST_DIR";
    let prj_dir: String = env::var(prj_dir).expect("Can't get {}", prj_dir);
    println!("cargo:rustc-link-search={}/target/", prj_dir);
    println!("cargo:rustc-link-lib=stat");
}

