pub mod ecx0x00000000;

use super::eax0x00000000::Eax0x00000000;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x00000007 {
    ecx0x00000000: ecx0x00000000::Ecx0x00000000,
}

impl Eax0x00000007 {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 7;
        if eax <= eax0x00000000.max_eax() {
            let ecx0x00000000 = ecx0x00000000::Ecx0x00000000::new();
            Some(Self {
                ecx0x00000000,
            })
        } else {
            None
        }
    }
}

