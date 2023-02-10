#[derive(Clone, Debug)]
pub struct GeneralFlags {
    allocation_possible: bool,
    no_fat_chain: bool,
}

impl GeneralFlags {
    pub fn file_name() -> Self {
        let allocation_possible = false;
        let no_fat_chain = false;
        Self {
            allocation_possible,
            no_fat_chain,
        }
    }

    pub fn stream_extension() -> Self {
        let allocation_possible = true;
        let no_fat_chain = false;
        Self {
            allocation_possible,
            no_fat_chain,
        }
    }

    pub fn volume_guid() -> Self {
        let allocation_possible = false;
        let no_fat_chain = true;
        Self {
            allocation_possible,
            no_fat_chain,
        }
    }
}

impl From<u8> for GeneralFlags {
    fn from(byte: u8) -> Self {
        let allocation_possible: bool = byte & 0x01 != 0;
        let no_fat_chain: bool = byte & 0x02 != 0;
        Self {
            allocation_possible,
            no_fat_chain,
        }
    }
}

impl Into<u8> for &GeneralFlags {
    fn into(self) -> u8 {
        let allocation_possible = if self.allocation_possible {
            1
        } else {
            0
        };
        let no_fat_chain = if self.no_fat_chain {
            2
        } else {
            0
        };
        allocation_possible + no_fat_chain
    }
}

