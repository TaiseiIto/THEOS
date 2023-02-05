use {
    std::{
        fmt,
        fs,
        mem,
        path::PathBuf,
    },
    super::super::super::super::binary::Binary,
};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct BootSector {
    jump_boot: [u8; 0x3],
    oem_name: [u8; 0x8],
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    fats: u8,
    root_directory_entries: u16,
    sectors16: u16,
    media: u8,
    sectors_per_fat: u16,
    sectors_per_track: u16,
    heads: u16,
    hidden_sectors: u32,
    sectors32: u32,
    drive_number: u8,
    reserved: u8,
    extended_boot_signature: u8,
    volume_identifier: u32,
    volume_label: [u8; 0xb],
    file_system_type: [u8; 0x8],
    boot_code: [u8; 0x1c0],
    boot_signature: u16,
}

impl BootSector {
    pub fn new(boot_sector: PathBuf) -> Self {
        let boot_sector: Vec<u8> = fs::read(&boot_sector).expect(&format!("Can't read {}!", boot_sector.display()));
        Self::read(&boot_sector)
    }

    pub fn read(bytes: &Vec<u8>) -> Self {
        const SIZE: usize = mem::size_of::<BootSector>();
        let boot_sector = &bytes[0..SIZE];
        let boot_sector: [u8; SIZE] = boot_sector.try_into().expect("Can't convert boot sector from Vec<u8> to [u8; SIZE]!");
        unsafe {
            mem::transmute::<[u8; SIZE], Self>(boot_sector)
        }
    }
}

impl Binary for BootSector {
    fn to_bytes(&self) -> Vec<u8> {
        let boot_sector: [u8; mem::size_of::<Self>()] = unsafe {
            mem::transmute::<Self, [u8; mem::size_of::<Self>()]>(*self)
        };
        let mut boot_sector: Vec<u8> = boot_sector.to_vec();
        boot_sector.resize(self.bytes_per_sector as usize, 0x00);
        boot_sector
    }
}

impl fmt::Display for BootSector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let jump_boot: String = "jump_boot:".to_string() + &self.jump_boot
            .iter()
            .map(|byte| format!(" {:02x}", byte))
            .fold(String::new(), |jump_boot, byte| jump_boot + &byte);
        let boot_sector: Vec<String> = vec![
            jump_boot,
        ];
        let boot_sector: String = boot_sector
            .into_iter()
            .fold(String::new(), |boot_sector, element| boot_sector + "\n" + &element);
        let boot_sector: String = boot_sector[1..].to_string();
        write!(f, "{}", boot_sector)
    }
}

