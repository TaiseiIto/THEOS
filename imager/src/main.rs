mod exfat;
mod guid;
mod mac_address;
mod rand;
mod time;

use std::{
    env,
    io,
    io::Write,
    path,
};

fn main() {
    let args = Args::new(env::args());
    eprintln!("{:#?}", args);
    let mut rand_generator = rand::Generator::new(time::Time::current_time().unix_timestamp() as u32);
    let exfat = exfat::Exfat::new(args.boot_sector, args.source_directory, &mut rand_generator);
    io::stdout().write(&exfat.to_bytes()).expect("Can't write image to stdout.");
}

#[derive(Debug)]
struct Args {
    boot_sector: path::PathBuf,
    source_directory: path::PathBuf,
}

impl Args {
    fn new(mut args: env::Args) -> Self {
        let usage: String = String::from("Usage: $ ./imager /path/to/boot/sector /path/to/source/directory");
        let _my_path: String = args.next().expect(&format!("{}\n{}\n", "Program path is not specified!", usage));
        let boot_sector: String = args.next().expect(&format!("{}\n{}\n", "Boot sector is not specified!", usage));
        let boot_sector = path::PathBuf::from(boot_sector);
        let source_directory: String = args.next().expect(&format!("{}\n{}\n", "Source directory is not specified!", usage));
        let source_directory = path::PathBuf::from(source_directory);
        Self {
            boot_sector,
            source_directory,
        }
    }
}

