use std::env;
use std::fs;
use std::mem;
use std::path;

fn main() {
    let args: Args = analyse_args(env::args());
    imager(args);
}

#[derive(Debug)]
struct Args {
    boot_sector: String,
    src: String,
    dst: String,
}

fn analyse_args(mut args: env::Args) -> Args {
    let usage: String = String::from("Usage: $ ./imager /path/to/boot/sector /path/to/source/directory /path/to/destination");
    let _my_path: String = args.next().expect(&format!("{}\n{}\n", "Program path is not specified.", usage));
    let boot_sector: String = args.next().expect(&format!("{}\n{}\n", "Boot sector is not specified.", usage));
    let src: String = args.next().expect(&format!("{}\n{}\n", "Source directory is not specified.", usage));
    let dst: String = args.next().expect(&format!("{}\n{}\n", "Boot sector is not specified.", usage));
    Args {
        boot_sector,
        src,
        dst,
    }
}

fn imager(args: Args) {
    let boot_sector = path::Path::new(&args.boot_sector);
    let src = path::Path::new(&args.src);
    let dst = path::Path::new(&args.dst);
    println!("src = {}", src.display());
    println!("dst = {}", dst.display());
    let boot_sector: BootSector = read_boot_sector(&boot_sector);
    println!("boot_sector.jump_boot = {:x?}", boot_sector.jump_boot);
}

const SECTOR_SIZE: usize = 0x200;
const JUMP_BOOT_SIZE: usize = 0x3;
const FILE_SYSTEM_NAME_SIZE: usize = 0x8;
const MUST_BE_ZERO_SIZE: usize = 0x35;

#[derive(Debug)]
#[repr(packed)]
struct BootSector {
    jump_boot: [u8; JUMP_BOOT_SIZE],
    unread: [u8; SECTOR_SIZE - JUMP_BOOT_SIZE],
}

fn read_boot_sector(boot_sector: &path::Path) -> BootSector {
    let boot_sector: Vec<u8> = fs::read(boot_sector).expect(&format!("Failed to open {}", boot_sector.display()));
    let boot_sector: [u8; SECTOR_SIZE] = boot_sector.try_into().expect(&format!("The length of boot sector must be {}.", SECTOR_SIZE));
    unsafe {
        mem::transmute::<[u8; SECTOR_SIZE], BootSector>(boot_sector)
    }
}

