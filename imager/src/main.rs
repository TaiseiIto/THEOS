mod args;
mod file_system;
mod guid;
mod mac_address;
mod rand;
mod time;

use std::{
    env,
    fs,
    io,
    io::Write,
};

fn main() {
    let mut rand_generator = rand::Generator::new(time::Time::current_time().unix_timestamp() as u32);
    match args::Args::new(env::args()) {
        args::Args::Read {
            image,
        } => {
            let image: Vec<u8> = fs::read(&image).expect(&format!("Can't read {}!", image.display()));
            let file_system = file_system::FileSystem::from(&image);
            println!("{}", file_system);
        },
        args::Args::Write {
            boot_sector,
            root_directory,
            has_volume_guid,
        } => {
            let file_system = file_system::FileSystem::new(boot_sector, root_directory, has_volume_guid, &mut rand_generator);
            eprintln!("{}", file_system);
            let file_system: Vec<u8> = (&file_system).into();
            io::stdout().write_all(&file_system).expect("Can't write image to stdout.");
        },
    }
}

