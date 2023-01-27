mod args;
mod binary;
mod exfat;
mod guid;
mod mac_address;
mod rand;
mod time;

use {
    binary::Binary,
    std::{
        env,
        fs,
        io,
        io::Write,
    },
};

fn main() {
    match args::Args::new(env::args()) {
        args::Args::Read {
            image,
        } => {
            let image: Vec<u8> = fs::read(&image).expect(&format!("Can't read {}!", image.display()));
            let exfat = exfat::Exfat::read(&image);
            println!("{}", exfat);
        },
        args::Args::Write {
            boot_sector,
            source_directory,
        } => {
            let mut rand_generator = rand::Generator::new(time::Time::current_time().unix_timestamp() as u32);
            let exfat = exfat::Exfat::new(boot_sector, source_directory, &mut rand_generator);
            let exfat: Vec<u8> = exfat.to_bytes();
            io::stdout().write_all(&exfat).expect("Can't write image to stdout.");
        },
    }
}

