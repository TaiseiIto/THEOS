use std::env;
use std::fs;
use std::path;

fn main() {
    let args: Args = match analyse_args(env::args()) {
        Ok(args) => args,
        Err(msg) => panic!("{}", msg),
    };
    if let Err(msg) = imager(args) {
        panic!("{}", msg);
    }
}

#[derive(Debug)]
struct Args {
    boot_sector: String,
    src: String,
    dst: String,
}

fn analyse_args(mut args: env::Args) -> Result<Args, String> {
    let usage: String = String::from("Usage: $ ./imager /path/to/boot/sector /path/to/source/directory /path/to/destination");
    let _my_path: String = match args.next() {
        Some(my_path) => my_path,
        None => return Err(format!("{}\n{}\n", "Program path is not specified.", usage)),
    };
    let boot_sector: String = match args.next() {
        Some(boot_sector) => boot_sector,
        None => return Err(format!("{}\n{}\n", "Boot sector is not specified.", usage)),
    };
    let src: String = match args.next() {
        Some(src) => src,
        None => return Err(format!("{}\n{}\n", "Source directory is not specified.", usage)),
    };
    let dst: String = match args.next() {
        Some(dst) => dst,
        None => return Err(format!("{}\n{}\n", "Boot sector is not specified.", usage)),
    };
    Ok(Args {
        boot_sector,
        src,
        dst,
    })
}

fn imager(args: Args) -> Result<(), String> {
    let boot_sector = path::Path::new(&args.boot_sector);
    let src = path::Path::new(&args.src);
    let dst = path::Path::new(&args.dst);
    if !boot_sector.exists() {
        return Err(String::from(format!("{} doesn't exist.", boot_sector.display())));
    }
    if boot_sector.is_dir() {
        return Err(String::from(format!("{} is directory.", boot_sector.display())));
    }
    if !src.exists() {
        return Err(String::from(format!("{} doesn't exist.", src.display())));
    }
    if src.is_file() {
        return Err(String::from(format!("{} is file.", src.display())));
    }
    if dst.exists() {
        return Err(String::from(format!("{} exists already.", dst.display())));
    }
    read_boot_sector(&boot_sector)?;
    println!("src = {}", src.display());
    println!("dst = {}", dst.display());
    Ok(())
}

fn read_boot_sector(boot_sector: &path::Path) -> Result<(), String> {
    let boot_sector: Vec<u8> = match fs::read(boot_sector) {
        Ok(boot_sector) => boot_sector,
        Err(_) => return Err(format!("Failed to open {}", boot_sector.display())),
    };
    let boot_sector_len: usize = boot_sector.len();
    if boot_sector_len != 0x200 {
        return Err(format!("Size of the boot sector must be 0x200 bytes but actually {} bytes.", boot_sector_len));
    }
    Ok(())
}

