use super::{
    mac_address,
    time,
};

#[derive(Debug)]
pub struct Guid {
    time: time::Time,
    version: u8,
    clock_sequence: u16,
    mac_address: u64,
}

impl Guid {
    pub fn new() -> Self {
        let time = time::Time::get_current_time();
        let version: u8 = 1;
        let clock_sequence: u16 = 0;
        let mac_address: u64 = mac_address::get_mac_address();
        Self {
            time,
            version,
            clock_sequence,
            mac_address,
        }
    }

    pub fn to_u128(self) -> u128 {
        let time: u128 = self.time.get_guid_timestamp() as u128;
        let version: u128 = (self.version as u128) << 0x3c;
        let clock_sequence: u128 = (self.clock_sequence as u128) << 0x40;
        let mac_address: u128 = (self.mac_address as u128) << 0x50;
        time + version + clock_sequence + mac_address
    }
}

