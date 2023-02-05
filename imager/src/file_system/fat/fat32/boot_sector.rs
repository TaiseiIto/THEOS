use {
    std::{
        fmt,
        fs,
        mem,
        path::PathBuf,
        str,
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
    sectors_per_fat16: u16,
    sectors_per_track: u16,
    heads: u16,
    hidden_sectors: u32,
    sectors32: u32,
    sectors_per_fat32: u32,
    fat_flags: u16,
    file_system_version: u16,
    root_directory_cluster: u32,
    file_system_information_sector: u16,
    backup_boot_sector: u16,
    reserved0: [u8; 0xc],
    drive_number: u8,
    reserved1: u8,
    extended_boot_signature: u8,
    volume_id: u32,
    volume_label: [u8; 0xb],
    file_system_type: [u8; 0x8],
    boot_code: [u8; 0x1a4],
    boot_signature: u16,
}

impl BootSector {
    pub fn new(boot_sector: &PathBuf) -> Self {
        let boot_sector: Vec<u8> = fs::read(boot_sector).expect(&format!("Can't read {}!", boot_sector.display()));
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
        let oem_name: String = format!("oem_name: \"{}\"", str::from_utf8(&self.oem_name).expect("Can't print a boot sector."));
        let bytes_per_sector: u16 = self.bytes_per_sector;
        let bytes_per_sector: String = format!("bytes_per_sector: {:#06x}", bytes_per_sector);
        let sectors_per_cluster: u8 = self.sectors_per_cluster;
        let sectors_per_cluster: String = format!("sectors_per_cluster: {:#04x}", sectors_per_cluster);
        let fats: u8 = self.fats;
        let fats: String = format!("fats: {:#04x}", fats);
        let root_directory_entries: u16 = self.root_directory_entries;
        let root_directory_entries: String = format!("root_directory_entries: {:#06x}", root_directory_entries);
        let sectors16: u16 = self.sectors16;
        let sectors16: String = format!("sectors16: {:#06x}", sectors16);
        let media: u8 = self.media;
        let media: String = format!("media: {:#04x}", media);
        let sectors_per_fat16: u16 = self.sectors_per_fat16;
        let sectors_per_fat16: String = format!("sectors_per_fat16: {:#06x}", sectors_per_fat16);
        let sectors_per_track: u16 = self.sectors_per_track;
        let sectors_per_track: String = format!("sectors_per_track: {:#06x}", sectors_per_track);
        let heads: u16 = self.heads;
        let heads: String = format!("heads: {:#06x}", heads);
        let hidden_sectors: u32 = self.hidden_sectors;
        let hidden_sectors: String = format!("hidden_sectors: {:#010x}", hidden_sectors);
        let sectors32: u32 = self.sectors32;
        let sectors32: String = format!("sectors32: {:#010x}", sectors32);
        let sectors_per_fat32: u32 = self.sectors_per_fat32;
        let sectors_per_fat32: String = format!("sectors_per_fat32: {:#010x}", sectors_per_fat32);
        let fat_flags: u16 = self.fat_flags;
        let fat_flags: String = format!("fat_flags: {:#06x}", fat_flags);
        let file_system_version: u16 = self.file_system_version;
        let file_system_version: String = format!("file_system_version: {:#06x}", file_system_version);
        let root_directory_cluster: u32 = self.root_directory_cluster;
        let root_directory_cluster: String = format!("root_directory_cluster: {:#010x}", root_directory_cluster);
        let file_system_information_sector: u16 = self.file_system_information_sector;
        let file_system_information_sector: String = format!("file_system_information_sector: {:#06x}", file_system_information_sector);
        let backup_boot_sector: u16 = self.backup_boot_sector;
        let backup_boot_sector: String = format!("backup_boot_sector: {:#06x}", backup_boot_sector);
        let reserved0: String = "reserved0:".to_string() + &self.reserved0
            .iter()
            .map(|byte| format!(" {:02x}", byte))
            .fold(String::new(), |reserved0, byte| reserved0 + &byte);
        let drive_number: u8 = self.drive_number;
        let drive_number: String = format!("drive_number: {:#04x}", drive_number);
        let reserved1: u8 = self.reserved1;
        let reserved1: String = format!("reserved: {:#04x}", reserved1);
        let extended_boot_signature: u8 = self.extended_boot_signature;
        let extended_boot_signature: String = format!("extended_boot_signature: {:#04x}", extended_boot_signature);
        let volume_id: u32 = self.volume_id;
        let volume_id: String = format!("volume_id: {:#010x}", volume_id);
        let volume_label: [u8; 0xb] = self.volume_label;
        let volume_label: String = format!("volume_label: \"{}\"", str::from_utf8(&volume_label).expect("Can't print a boot sector."));
        let file_system_type: [u8; 0x8] = self.file_system_type;
        let file_system_type: String = format!("file_system_type: \"{}\"", str::from_utf8(&file_system_type).expect("Can't print a boot sector."));
        let boot_code: String = "boot_code:".to_string() + &self.boot_code
            .iter()
            .map(|byte| format!(" {:02x}", byte))
            .fold(String::new(), |boot_code, byte| boot_code + &byte);
        let boot_signature: u16 = self.boot_signature;
        let boot_signature: String = format!("boot_signature: {:#06x}", boot_signature);
        let boot_sector: Vec<String> = vec![
            jump_boot,
            oem_name,
            bytes_per_sector,
            sectors_per_cluster,
            fats,
            root_directory_entries,
            sectors16,
            media,
            sectors_per_fat16,
            sectors_per_track,
            heads,
            hidden_sectors,
            sectors32,
            sectors_per_fat32,
            fat_flags,
            file_system_version,
            root_directory_cluster,
            file_system_information_sector,
            backup_boot_sector,
            reserved0,
            drive_number,
            reserved1,
            extended_boot_signature,
            volume_id,
            volume_label,
            file_system_type,
            boot_code,
            boot_signature,
        ];
        let boot_sector: String = boot_sector
            .into_iter()
            .fold(String::new(), |boot_sector, element| boot_sector + "\n" + &element);
        let boot_sector: String = boot_sector[1..].to_string();
        write!(f, "{}", boot_sector)
    }
}

