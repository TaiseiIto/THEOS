use std::env;
use std::path::Path;

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
    src_dir: String,
    destination: String,
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
    let src_dir: String = match args.next() {
        Some(src_dir) => src_dir,
        None => return Err(format!("{}\n{}\n", "Source directory is not specified.", usage)),
    };
    let destination: String = match args.next() {
        Some(destination) => destination,
        None => return Err(format!("{}\n{}\n", "Boot sector is not specified.", usage)),
    };
    Ok(Args {
        boot_sector,
        src_dir,
        destination,
    })
}

fn imager(args: Args) -> Result<(), String> {
    let boot_sector = Path::new(&args.boot_sector);
    let src_dir = Path::new(&args.src_dir);
    let destination = Path::new(&args.destination);
    if !boot_sector.exists() {
        return Err(String::from(format!("{} doesn't exist.", boot_sector.display())));
    }
    if boot_sector.is_dir() {
        return Err(String::from(format!("{} is directory.", boot_sector.display())));
    }
    if !src_dir.exists() {
        return Err(String::from(format!("{} doesn't exist.", src_dir.display())));
    }
    if src_dir.is_file() {
        return Err(String::from(format!("{} is file.", src_dir.display())));
    }
    if destination.exists() {
        return Err(String::from(format!("{} exists already.", destination.display())));
    }
    println!("boot_sector = {}", boot_sector.display());
    println!("src_dir = {}", src_dir.display());
    println!("destination = {}", destination.display());
    Ok(())
}

