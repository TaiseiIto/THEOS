mod exfat;
mod time;

use std::{
    env,
    fmt,
    path,
};

fn main() {
    let args = Args::new(env::args());
    println!("{}", args);
    let exfat = exfat::Exfat::new(
        args.boot_sector,
        args.src,
    );
    println!("{}", exfat);
    exfat.dump(args.dst);
}

#[derive(Debug)]
struct Args {
    boot_sector: path::PathBuf,
    src: path::PathBuf,
    dst: path::PathBuf,
}

impl Args {
    fn new(mut args: env::Args) -> Self {
        let usage: String = String::from("Usage: $ ./imager /path/to/boot/sector /path/to/source/directory /path/to/destination");
        let _my_path: String = args.next().expect(&format!("{}\n{}\n", "Program path is not specified.", usage));
        let boot_sector: String = args.next().expect(&format!("{}\n{}\n", "Boot sector is not specified.", usage));
        let boot_sector: path::PathBuf = path::PathBuf::from(boot_sector);
        let src: String = args.next().expect(&format!("{}\n{}\n", "Source directory is not specified.", usage));
        let src: path::PathBuf = path::PathBuf::from(src);
        let dst: String = args.next().expect(&format!("{}\n{}\n", "Boot sector is not specified.", usage));
        let dst: path::PathBuf = path::PathBuf::from(dst);
        Self {
            boot_sector,
            src,
            dst,
        }
    }
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "arg.boot_sector = {}\n", self.boot_sector.display())?;
        write!(f, "arg.src = {}\n", self.src.display())?;
        write!(f, "arg.dst = {}", self.dst.display())
    }
}

