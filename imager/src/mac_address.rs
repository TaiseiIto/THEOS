use std::{
    fs,
    path,
    str,
};

#[derive(Clone, Copy, Debug)]
pub struct MacAddress {
    address: u64
}

impl MacAddress {
    pub fn get_address(&self) -> u64 {
        self.address
    }

    pub fn me() -> Self {
        let address: &str = "/sys/class/net/eth0/address";
        let address = path::PathBuf::from(address);
        let address: Vec<u8> = fs::read(address).expect("Can't read MAC address.");
        let address: &str = str::from_utf8(&address).expect("Can't read MAC address.");
        let address: String = address.to_lowercase();
        let (mut address, hex): (Vec<String>, String) = address
            .chars()
            .fold((vec![] as Vec<String>, String::new()), |(address, hex), c| match c {
                '0' |
                '1' |
                '2' |
                '3' |
                '4' |
                '5' |
                '6' |
                '7' |
                '8' |
                '9' |
                'a' |
                'b' |
                'c' |
                'd' |
                'e' |
                'f' => {
                    let mut hex: String = hex;
                    hex.push(c);
                    (address, hex)
                },
                _ => {
                    let mut address: Vec<String> = address;
                    address.push(hex);
                    (address, String::new())
                }
            });
        address.push(hex);
        let address: Vec<String> = address
            .into_iter()
            .filter(|hex| 0 < hex.len())
            .collect();
        let address: Vec<u64> = address
            .into_iter()
            .map(|hex| hex.chars().fold(0, |hex, c| {
                let c: u64 = match c {
                    '0' => 0x0,
                    '1' => 0x1,
                    '2' => 0x2,
                    '3' => 0x3,
                    '4' => 0x4,
                    '5' => 0x5,
                    '6' => 0x6,
                    '7' => 0x7,
                    '8' => 0x8,
                    '9' => 0x9,
                    'a' => 0xa,
                    'b' => 0xb,
                    'c' => 0xc,
                    'd' => 0xd,
                    'e' => 0xe,
                    'f' => 0xf, 
                    _ => 0x0,
                };
                (hex << 4) + c
            }))
            .collect();
        let address: u64 = address
            .into_iter()
            .fold(0, |address, hex| (address << 8) + hex);
        Self {
            address,
        }
    }

    pub fn new(address: u64) -> Self {
        Self {
            address,
        }
    }

    pub fn null() -> Self {
        let address: u64 = 0;
        Self {
            address,
        }
    }
}
