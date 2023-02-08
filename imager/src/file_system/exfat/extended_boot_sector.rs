use {
    std::{
        convert::{
            From,
            Into,
        },
        fmt,
        mem,
    },
};

#[derive(Clone, Copy, Debug)]
pub struct ExtendedBootSector {
    extended_boot_signature: u32,
    size: usize,
}

impl ExtendedBootSector {
    pub fn new(size: usize) -> Self {
        let extended_boot_signature: u32 = 0xaa550000;
        Self {
            extended_boot_signature,
            size,
        }
    }
}

impl From<&Vec<u8>> for ExtendedBootSector {
    fn from(bytes: &Vec<u8>) -> Self {
        let size: usize = bytes.len();
        let extended_boot_signature: Vec<u8> = bytes[size - mem::size_of::<u32>()..].to_vec();
        let extended_boot_signature: [u8; mem::size_of::<u32>()] = extended_boot_signature.try_into().expect("Can't read extended boot signature.");
        let extended_boot_signature: u32 = unsafe {
            mem::transmute::<[u8; mem::size_of::<u32>()], u32>(extended_boot_signature)
        };
        Self {
            extended_boot_signature,
            size,
        }
    }
}

impl Into<Vec<u8>> for &ExtendedBootSector {
    fn into(self) -> Vec<u8> {
        let mut extended_boot_signature: Vec<u8> = unsafe {
            mem::transmute::<u32, [u8; 4]>(self.extended_boot_signature)
        }.to_vec();
        let extended_boot_code_size: usize = self.size - extended_boot_signature.len();
        let extended_boot_code: Vec<u8> = (0..extended_boot_code_size)
            .map(|_| 0)
            .collect();
        let mut extended_boot_sector: Vec<u8> = extended_boot_code;
        extended_boot_sector.append(&mut extended_boot_signature);
        extended_boot_sector
    }
}

impl fmt::Display for ExtendedBootSector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let extended_boot_signature: String = format!("boot_signature: {:#010x}", self.extended_boot_signature);
        let extended_boot_sector_size: String = format!("sector_size: {:#x}", self.size);
        let extended_boot_sector: String = format!("{}\n{}", extended_boot_signature, extended_boot_sector_size);
        write!(f, "{}", extended_boot_sector)
    }
}

