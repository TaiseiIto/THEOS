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
    match Args::new(env::args()) {
        Args::Read {
            image,
        } => {
            println!("Read {}", image.display());
        },
        Args::Write {
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

#[derive(Debug)]
enum Args {
    Read {
        image: path::PathBuf,
    },
    Write {
        boot_sector: path::PathBuf,
        source_directory: path::PathBuf,
    },
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
        let image: Option<&String> = args.get("-i");
        if let Some(image) = image {
            let image = path::PathBuf::from(image);
            Self::Read {
                image,
            }
        } else if let (Some(boot_sector), Some(source_directory)) = (boot_sector, source_directory) {
            let boot_sector = path::PathBuf::from(boot_sector);
            let source_directory = path::PathBuf::from(source_directory);
            Self::Write {
                boot_sector,
                source_directory,
            }
        } else {
            panic!("Can't interpret args.");
        }
    }
}

