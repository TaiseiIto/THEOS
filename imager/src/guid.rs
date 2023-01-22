use {
    std::mem,
    super::{
        mac_address,
        rand,
        time,
    }
};

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

    pub fn read(bytes: &Vec<u8>) -> (Self, usize) {
        let guid: Vec<Vec<u8>> = bytes
            .chunks(mem::size_of::<u128>())
            .map(|chunk| chunk.to_vec())
            .collect();
        let guid: [u8; mem::size_of::<u128>()] = guid[0]
            .clone()
            .try_into()
            .expect("Can't read GUID.");
        let guid: u128 = unsafe {
            mem::transmute::<[u8; mem::size_of::<u128>()], u128>(guid)
        };
        let time_and_version: u64 = guid as u64;
        let time: u64 = time_and_version & 0x0fffffffffffffff;
        let time = time::Time::from_guid_timestamp(time);
        let version: u8 = (time_and_version << 60) as u8;
        let clock_sequence: u16 = (time_and_version << 64) as u16;
        let mac_address: u64 = (time_and_version << 80) as u64;
        let guid = Self {
            clock_sequence,
            mac_address,
            time,
            version,
        };
        let size: usize = mem::size_of::<u128>();
        (guid, size)
    }

    pub fn to_u128(&self) -> u128 {
        let clock_sequence: u128 = (self.clock_sequence as u128) << 0x40;
        let mac_address: u128 = (self.mac_address as u128) << 0x50;
        let time: u128 = self.time.guid_timestamp() as u128;
        let version: u128 = (self.version as u128) << 0x3c;
        clock_sequence + mac_address + time + version
    }
}

