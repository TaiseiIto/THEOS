use {
    std::{
        collections::HashMap,
        env,
        path::PathBuf,
    },
};

#[derive(Debug)]
pub enum Args {
    Read {
        image: PathBuf,
    },
    Write {
        boot_sector: Vec<PathBuf>,
        root_directory: PathBuf,
        has_volume_guid: bool,
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
        let has_volume_guid: Option<&String> = args.get("-v");
        let image: Option<&String> = args.get("-i");
        match (boot_sector, root_directory, has_volume_guid, image) {
            (Some(boot_sector), Some(root_directory), Some(has_volume_guid), None) => {
                let boot_sector: Vec<PathBuf> = boot_sector
                    .split(',')
                    .map(|boot_sector| PathBuf::from(boot_sector))
                    .collect();
                let root_directory = PathBuf::from(root_directory);
                let has_volume_guid: char = has_volume_guid
                    .chars()
                    .next()
                    .expect("Can't interpret args.")
                    .to_uppercase()
                    .next()
                    .expect("Can't interpret args.");
                let has_volume_guid: bool = match has_volume_guid {
                    'T' => true,
                    'F' => false,
                    _ => panic!("Can't interpret args."),
                };
                Self::Write {
                    boot_sector,
                    root_directory,
                    has_volume_guid,
                }
            },
            (None, None, None, Some(image)) => {
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

