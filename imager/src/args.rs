use std::{
    collections::HashMap,
    env,
    path,
};

#[derive(Debug)]
pub enum Args {
    Read {
        image: path::PathBuf,
    },
    Write {
        boot_sector: path::PathBuf,
        source_directory: path::PathBuf,
    },
}

impl Args {
    pub fn new(mut args: env::Args) -> Self {
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

