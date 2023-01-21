mod binary;
mod exfat;
mod guid;
mod mac_address;
mod rand;
mod time;

use {
    binary::Binary,
    std::{
        collections::HashMap,
        env,
        io,
        io::Write,
        path,
    },
};

fn main() {
    let args = Args::new(env::args());
    let mut rand_generator = rand::Generator::new(time::Time::current_time().unix_timestamp() as u32);
    let exfat = exfat::Exfat::new(args.boot_sector, args.source_directory, &mut rand_generator);
    let exfat: Vec<u8> = exfat.to_bytes();
    io::stdout().write_all(&exfat).expect("Can't write image to stdout.");
}

#[derive(Debug)]
struct Args {
    boot_sector: path::PathBuf,
    source_directory: path::PathBuf,
}

impl Args {
    fn new(mut args: env::Args) -> Self {
        args.next();
        let args: Vec<String> = args.collect();
        let args: HashMap<String, String> = args
            .chunks(2)
            .filter_map(|tuple| match tuple {
                [key, value] => Some((key.clone(), value.clone())),
                _ => None,
            })
            .collect();
        let boot_sector: Option<&String> = args.get("-b");
        let source_directory: Option<&String> = args.get("-s");
        if let (Some(boot_sector), Some(source_directory)) = (boot_sector, source_directory) {
            let boot_sector = path::PathBuf::from(boot_sector);
            let source_directory = path::PathBuf::from(source_directory);
            Self {
                boot_sector,
                source_directory,
            }
        } else {
            panic!("Can't interpret args.");
        }
    }
}

