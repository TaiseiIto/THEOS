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
        match (boot_sector, source_directory, image) {
            (Some(boot_sector), Some(source_directory), _) => {
                let boot_sector = path::PathBuf::from(boot_sector);
                let source_directory = path::PathBuf::from(source_directory);
                Self::Write {
                    boot_sector,
                    source_directory,
                }
            },
            (_, _, Some(image)) => {
                let image = path::PathBuf::from(image);
                Self::Read {
                    image,
                }
            },
            _ => {
                panic!("Can't interpret args.");
            }
        }
    }
}

