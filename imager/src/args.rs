use std::{
    collections::HashMap,
    convert,
    env,
    path::PathBuf,
    str,
};

#[derive(Debug)]
pub enum Args {
    Read {
        image: PathBuf,
    },
    Write {
        boot_sector: PathBuf,
        file_system: FileSystem,
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
        let file_system: Option<&String> = args.get("-f");
        let root_directory: Option<&String> = args.get("-r");
        let image: Option<&String> = args.get("-i");
        match (boot_sector, file_system, root_directory, image) {
            (Some(boot_sector), Some(file_system), Some(root_directory), _) => {
                let boot_sector = PathBuf::from(boot_sector);
                let file_system: FileSystem = file_system.parse().expect("Can't interpret args.");
                let root_directory = PathBuf::from(root_directory);
                Self::Write {
                    boot_sector,
                    file_system,
                    root_directory,
                }
            },
            (_, _, _, Some(image)) => {
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

#[derive(Debug)]
pub enum FileSystem {
    ExFat,
    Fat,
}

impl str::FromStr for FileSystem {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}

impl convert::TryFrom<&str> for FileSystem {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s: String = s.to_lowercase();
        match &*s {
            "exfat" => Ok(Self::ExFat),
            "fat" => Ok(Self::Fat),
            _ => Err("Can't interpret file system."),
        }
    }
}

