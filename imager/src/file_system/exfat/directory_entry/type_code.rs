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

impl From<u8> for TypeCode {
    fn from(byte: u8) -> Self {
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
}

impl Into<u8> for &TypeCode {
    fn into(self) -> u8 {
        match self {
            TypeCode::File => 0x05,
            TypeCode::StreamExtension => 0x00,
            TypeCode::FileName => 0x01,
            TypeCode::UpcaseTable => 0x02,
            TypeCode::VolumeLabel => 0x03,
            TypeCode::VolumeGuid => 0x00,
            TypeCode::AllocationBitmap => 0x01,
        }
    }
}

