
#[derive(Debug)]
pub enum TypeCode {
    File,
    StreamExtension,
    FileName,
    UpcaseTable,
    VolumeLabel,
    VolumeGuid,
    AllocationBitmap,
}

impl TypeCode {
    pub fn read(byte: u8) -> Self {
        let type_code: u8 = byte & 0x1f;
        let type_category: bool = byte & 0x40 != 0;
        match type_code {
            0x00 => if type_category {
                Self::StreamExtension
            } else {
                Self::VolumeGuid
            },
            0x01 => if type_category {
                Self::FileName
            } else {
                Self::AllocationBitmap
            },
            0x02 => Self::UpcaseTable,
            0x03 => Self::VolumeLabel,
            0x05 => Self::File,
            _ => panic!("Can't read type code."),
        }
    }

    pub fn to_byte(&self) -> u8 {
        match self {
            Self::File => 0x05,
            Self::StreamExtension => 0x00,
            Self::FileName => 0x01,
            Self::UpcaseTable => 0x02,
            Self::VolumeLabel => 0x03,
            Self::VolumeGuid => 0x00,
            Self::AllocationBitmap => 0x01,
        }
    }
}

