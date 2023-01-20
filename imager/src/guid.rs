use super::{
    mac_address,
    rand,
    time,
};

#[derive(Debug)]
pub struct Guid {
    clock_sequence: u16,
    mac_address: u64,
    time: time::Time,
    version: u8,
}

impl Guid {
    pub fn new(rand_generator: &mut rand::Generator) -> Self {
        let clock_sequence: u16 = rand_generator.generate_u16();
        let mac_address: u64 = mac_address::my_mac_address();
        let time = time::Time::current_time();
        let version: u8 = 1;
        Self {
            clock_sequence,
            mac_address,
            time,
            version,
        }
    }

    pub fn to_u128(self) -> u128 {
        let clock_sequence: u128 = (self.clock_sequence as u128) << 0x40;
        let mac_address: u128 = (self.mac_address as u128) << 0x50;
        let time: u128 = self.time.guid_timestamp() as u128;
        let version: u128 = (self.version as u128) << 0x3c;
        clock_sequence + mac_address + time + version
    }
}

