use {
    std::{
        fmt,
        fs,
        mem,
        path::PathBuf,
        str,
    },
};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Fat16 {
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
    volume_id: u32,
    volume_label: [u8; 0xb],
    file_system_type: [u8; 0x8],
    boot_code: [u8; 0x1c0],
    boot_signature: u16,
}

impl Fat16 {
    pub fn cluster_size(&self) -> usize {
        self.bytes_per_sector as usize * self.sectors_per_cluster as usize
    }

    pub fn fats(&self) -> usize {
        self.fats as usize
    }

    pub fn media(&self) -> u8 {
        self.media
    }

    pub fn reserved_sectors(&self) -> usize {
        self.reserved_sectors as usize
    }

    pub fn root_directory_entries(&self) -> Option<usize> {
        Some(self.root_directory_entries as usize)
    }

    pub fn sector_size(&self) -> usize {
        self.bytes_per_sector as usize
    }

    pub fn sectors_per_fat(&self) -> usize {
        self.sectors_per_fat as usize
    }

    pub fn volume_label(&self) -> [u8; 0xb] {
        self.volume_label
    }
}

impl From<&PathBuf> for Fat16 {
    fn from(boot_sector: &PathBuf) -> Self {
        let boot_sector: &Vec<u8> = &fs::read(boot_sector).expect(&format!("Can't read {}!", boot_sector.display()));
        boot_sector.into()
    }
}

impl From<&Vec<u8>> for Fat16 {
    fn from(bytes: &Vec<u8>) -> Self {
        const SIZE: usize = mem::size_of::<Fat16>();
        let boot_sector = &bytes[0..SIZE];
        let boot_sector: [u8; SIZE] = boot_sector.try_into().expect("Can't convert boot sector from Vec<u8> to [u8; SIZE]!");
        unsafe {
            mem::transmute::<[u8; SIZE], Self>(boot_sector)
        }
    }
}

impl Into<Vec<u8>> for &Fat16 {
    fn into(self) -> Vec<u8> {
        let boot_sector: [u8; mem::size_of::<Fat16>()] = unsafe {
            mem::transmute::<Fat16, [u8; mem::size_of::<Fat16>()]>(*self)
        };
        let mut boot_sector: Vec<u8> = boot_sector.to_vec();
        boot_sector.resize(self.bytes_per_sector as usize, 0x00);
        boot_sector
    }
}

impl fmt::Display for Fat16 {
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
        let sectors_per_fat: u16 = self.sectors_per_fat;
        let sectors_per_fat: String = format!("sectors_per_fat: {:#06x}", sectors_per_fat);
        let sectors_per_track: u16 = self.sectors_per_track;
        let sectors_per_track: String = format!("sectors_per_track: {:#06x}", sectors_per_track);
        let heads: u16 = self.heads;
        let heads: String = format!("heads: {:#06x}", heads);
        let hidden_sectors: u32 = self.hidden_sectors;
        let hidden_sectors: String = format!("hidden_sectors: {:#010x}", hidden_sectors);
        let sectors32: u32 = self.sectors32;
        let sectors32: String = format!("sectors32: {:#010x}", sectors32);
        let drive_number: u8 = self.drive_number;
        let drive_number: String = format!("drive_number: {:#04x}", drive_number);
        let reserved: u8 = self.reserved;
        let reserved: String = format!("reserved: {:#04x}", reserved);
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
            sectors_per_fat,
            sectors_per_track,
            heads,
            hidden_sectors,
            sectors32,
            drive_number,
            reserved,
            extended_boot_signature,
            volume_id,
            volume_label,
            file_system_type,
            boot_code,
            boot_signature,
        ];
        let boot_sector: String = boot_sector.join("\n");
        write!(f, "{}", boot_sector)
    }
}

