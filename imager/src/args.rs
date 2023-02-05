use std::{
    collections::HashMap,
    env,
    path::PathBuf,
};

#[derive(Debug)]
pub enum Args {
    Read {
        image: PathBuf,
    },
    Write {
        boot_sector: PathBuf,
        root_directory: PathBuf,
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
        let root_directory: Option<&String> = args.get("-r");
        let image: Option<&String> = args.get("-i");
        match (boot_sector, root_directory, image) {
            (Some(boot_sector), Some(root_directory), _) => {
                let boot_sector = PathBuf::from(boot_sector);
                let root_directory = PathBuf::from(root_directory);
                Self::Write {
                    boot_sector,
                    root_directory,
                }
            },
            (_, _, Some(image)) => {
                let image = PathBuf::from(image);
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

