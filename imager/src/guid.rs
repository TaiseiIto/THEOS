use {
    std::{
        fmt,
        mem,
    },
    super::{
        mac_address,
        rand,
        time,
    }
};

pub const GUID_SIZE: usize = mem::size_of::<u128>();

#[derive(Clone, Copy, Debug)]
pub struct Guid {
    clock_sequence: u16,
    mac_address: mac_address::MacAddress,
    time: time::Time,
    version: u8,
}

impl Guid {
    pub fn new(rand_generator: &mut rand::Generator) -> Self {
        let clock_sequence: u16 = rand_generator.generate_u16();
        let mac_address = mac_address::MacAddress::me();
        let time = time::Time::current_time();
        let version: u8 = 1;
        Self {
            clock_sequence,
            mac_address,
            time,
            version,
        }
    }

    pub fn null() -> Self {
        let clock_sequence: u16 = 0;
        let mac_address = mac_address::MacAddress::null();
        let time = time::Time::new(1582, 10, 15, 0, 0, 0, 0);
        let version: u8 = 0;
        Self {
            clock_sequence,
            mac_address,
            time,
            version,
        }
    }

    pub fn read(guid: u128) -> Self {
        let time_and_version: u64 = guid as u64;
        let time: u64 = time_and_version & 0x0fffffffffffffff;
        let time = time::Time::from_guid_timestamp(time);
        let version: u8 = (time_and_version >> 60) as u8;
        let clock_sequence: u16 = (guid >> 64) as u16;
        let mac_address: u64 = (guid >> 80) as u64;
        let mac_address = mac_address::MacAddress::new(mac_address);
        Self {
            clock_sequence,
            mac_address,
            time,
            version,
        }
    }

    pub fn to_u128(&self) -> u128 {
        let clock_sequence: u128 = (self.clock_sequence as u128) << 0x40;
        let mac_address: u128 = (self.mac_address.address() as u128) << 0x50;
        let time: u128 = self.time.guid_timestamp() as u128;
        let version: u128 = (self.version as u128) << 0x3c;
        clock_sequence + mac_address + time + version
    }
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clock_sequence: String = format!("clock_sequence: {:#06x}", self.clock_sequence);
        let mac_address: String = format!("mac_address: {}", self.mac_address);
        let time: String = format!("time: {}", self.time);
        let version: String = format!("version: {:#04x}", self.version);
        let guid: Vec<String> = vec![
            version,
            mac_address,
            time,
            clock_sequence,
        ];
        let guid: String = guid
            .join("\n");
        write!(f, "{}", guid)
    }
}

