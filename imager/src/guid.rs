use {
    std::mem,
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

    pub fn null() -> Self {
        let clock_sequence: u16 = 0;
        let mac_address: u64 = 0;
        let time = time::Time::new(1970, 1, 1, 0, 0, 0, 0);
        let version: u8 = 0;
        Self {
            clock_sequence,
            mac_address,
            time,
            version,
        }
    }

    pub fn read(bytes: &Vec<u8>) -> Self {
        let guid: Vec<u8> = bytes
            .chunks(GUID_SIZE)
            .next()
            .expect("Can't read GUID.")
            .to_vec();
        let guid: [u8; GUID_SIZE] = guid
            .try_into()
            .expect("Can't read GUID.");
        let guid: u128 = unsafe {
            mem::transmute::<[u8; GUID_SIZE], u128>(guid)
        };
        let time_and_version: u64 = guid as u64;
        let time: u64 = time_and_version & 0x0fffffffffffffff;
        let time = time::Time::from_guid_timestamp(time);
        let version: u8 = (time_and_version << 60) as u8;
        let clock_sequence: u16 = (time_and_version << 64) as u16;
        let mac_address: u64 = (time_and_version << 80) as u64;
        Self {
            clock_sequence,
            mac_address,
            time,
            version,
        }
    }

    pub fn to_u128(&self) -> u128 {
        let clock_sequence: u128 = (self.clock_sequence as u128) << 0x40;
        let mac_address: u128 = (self.mac_address as u128) << 0x50;
        let time: u128 = self.time.guid_timestamp() as u128;
        let version: u128 = (self.version as u128) << 0x3c;
        clock_sequence + mac_address + time + version
    }
}

