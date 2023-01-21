use std::{
    fs,
    path,
    str,
};

pub fn my_mac_address() -> u64 {
    let mac_address: &str = "/sys/class/net/eth0/address";
    let mac_address = path::PathBuf::from(mac_address);
    let mac_address: Vec<u8> = fs::read(mac_address).expect("Can't read MAC address.");
    let mac_address: &str = str::from_utf8(&mac_address).expect("Can't read MAC address.");
    let mac_address: String = mac_address.to_lowercase();
    let (mut mac_address, hex): (Vec<String>, String) = mac_address
        .chars()
        .fold((vec![] as Vec<String>, String::new()), |(mac_address, hex), c| match c {
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
                (mac_address, hex)
            },
            _ => {
                let mut mac_address: Vec<String> = mac_address;
                mac_address.push(hex);
                (mac_address, String::new())
            }
        });
    mac_address.push(hex);
    let mac_address: Vec<String> = mac_address
        .into_iter()
        .filter(|hex| 0 < hex.len())
        .collect();
    let mac_address: Vec<u64> = mac_address
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
    let mac_address: u64 = mac_address
        .into_iter()
        .fold(0, |mac_address, hex| (mac_address << 8) + hex);
    mac_address
}

