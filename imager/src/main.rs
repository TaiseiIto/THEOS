use std::env;
use std::fmt;
use std::path;

mod exfat;

fn main() {
    let args = Args::new(env::args());
    println!("{}", args);
    let exfat = exfat::Exfat::new(
        path::Path::new(&args.boot_sector),
        path::Path::new(&args.src),
    );
    println!("{}", exfat);
    exfat.dump(path::Path::new(&args.dst));
}

#[derive(Debug)]
struct Args {
    boot_sector: String,
    src: String,
    dst: String,
}

impl Args {
    fn new(mut args: env::Args) -> Self {
        let usage: String = String::from("Usage: $ ./imager /path/to/boot/sector /path/to/source/directory /path/to/destination");
        let _my_path: String = args.next().expect(&format!("{}\n{}\n", "Program path is not specified.", usage));
        let boot_sector: String = args.next().expect(&format!("{}\n{}\n", "Boot sector is not specified.", usage));
        let src: String = args.next().expect(&format!("{}\n{}\n", "Source directory is not specified.", usage));
        let dst: String = args.next().expect(&format!("{}\n{}\n", "Boot sector is not specified.", usage));
        Self {
            boot_sector,
            src,
            dst,
        }
    }
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "arg.boot_sector = {}\n", self.boot_sector)?;
        write!(f, "arg.src = {}\n", self.src)?;
        write!(f, "arg.dst = {}", self.dst)
    }
}

