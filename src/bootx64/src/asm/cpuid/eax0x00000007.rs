pub mod ecx0x00000000;
pub mod ecx0x00000001;

use super::eax0x00000000::Eax0x00000000;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x00000007 {
    ecx0x00000000: ecx0x00000000::Ecx0x00000000,
    ecx0x00000001: ecx0x00000001::Ecx0x00000001,
}

impl Eax0x00000007 {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 7;
        if eax <= eax0x00000000.max_eax() {
            let ecx0x00000000 = ecx0x00000000::Ecx0x00000000::new();
            let ecx0x00000001 = ecx0x00000001::Ecx0x00000001::new();
            Some(Self {
                ecx0x00000000,
                ecx0x00000001,
            })
        } else {
            None
        }
    }

    pub fn supports_5_level_paging(&self) -> bool {
        self.ecx0x00000000.supports_5_level_paging()
    }
}

