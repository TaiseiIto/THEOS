use std::env;
use std::fs;
use std::path;

fn main() {
    let args: Args = match analyse_args(env::args()) {
        Ok(args) => args,
        Err(msg) => panic!("{}", msg),
    };
    imager(args)
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

fn imager(args: Args) {
    let boot_sector = path::Path::new(&args.boot_sector);
    let src = path::Path::new(&args.src);
    let dst = path::Path::new(&args.dst);
    println!("src = {}", src.display());
    println!("dst = {}", dst.display());
    let boot_sector: BootSector = read_boot_sector(&boot_sector);
    println!("boot_sector.jump_boot = {:x?}", boot_sector.jump_boot);
    println!("boot_sector.file_system_name = {:?}", boot_sector.file_system_name);
    println!("boot_sector.must_be_zero = {:x?}", boot_sector.must_be_zero);
}

const SECTOR_SIZE: usize = 0x200;
const JUMP_BOOT_SIZE: usize = 0x3;
const FILE_SYSTEM_NAME_SIZE: usize = 0x8;
const MUST_BE_ZERO_SIZE: usize = 0x35;

#[derive(Debug)]
struct BootSector {
    jump_boot: [u8; JUMP_BOOT_SIZE],
    file_system_name: [char; FILE_SYSTEM_NAME_SIZE],
    must_be_zero: [u8; MUST_BE_ZERO_SIZE],
}

fn read_boot_sector(boot_sector: &path::Path) -> BootSector {
    let boot_sector: Vec<u8> = fs::read(boot_sector).expect(&format!("Failed to open {}", boot_sector.display()));
    let boot_sector: [u8; SECTOR_SIZE] = boot_sector.try_into().expect(&format!("The length of boot sector must be {}.", SECTOR_SIZE));
    let mut offset: usize = 0;
    let jump_boot: [u8; JUMP_BOOT_SIZE] = boot_sector[offset..offset + JUMP_BOOT_SIZE].try_into().expect("Can't read JumpBoot.");
    offset += JUMP_BOOT_SIZE;
    let file_system_name: [u8; FILE_SYSTEM_NAME_SIZE] = boot_sector[offset..offset + FILE_SYSTEM_NAME_SIZE].try_into().expect("Can't read FileSystemName.");
    let file_system_name: [char; FILE_SYSTEM_NAME_SIZE] = file_system_name.iter().map(|c| char::from(*c)).collect::<Vec<char>>().try_into().expect("Can't interpret FileSystemName as [char; FILE_SYSTEM_NAME_SIZE].");
    offset += FILE_SYSTEM_NAME_SIZE;
    let must_be_zero: [u8; MUST_BE_ZERO_SIZE] = boot_sector[offset..offset + MUST_BE_ZERO_SIZE].try_into().expect("Can't read MustBeZero.");
    BootSector {
        jump_boot,
        file_system_name,
        must_be_zero,
    }
}

