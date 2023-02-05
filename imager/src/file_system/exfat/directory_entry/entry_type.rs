use super::type_code;

#[derive(Debug)]
pub struct EntryType {
    type_code: type_code::TypeCode,
    type_importance: bool,
    type_category: bool,
    in_use: bool,
}

impl EntryType {
    pub fn allocation_bitmap() -> Self {
        let type_code = type_code::TypeCode::AllocationBitmap;
        let type_importance = false;
        let type_category = false;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }

    pub fn file() -> Self {
        let type_code = type_code::TypeCode::File;
        let type_importance = false;
        let type_category = false;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }

    pub fn file_name() -> Self {
        let type_code = type_code::TypeCode::FileName;
        let type_importance = false;
        let type_category = true;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }

    pub fn stream_extension() -> Self {
        let type_code = type_code::TypeCode::StreamExtension;
        let type_importance = false;
        let type_category = true;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }

    pub fn to_byte(&self) -> u8 {
        let type_code: u8 = self.type_code.to_byte();
        let type_importance: u8 = if self.type_importance {
            1 << 5
        } else {
            0
        };
        let type_category: u8 = if self.type_category {
            1 << 6
        } else {
            0
        };
        let in_use: u8 = if self.in_use {
            1 << 7
        } else {
            0
        };
        type_code + type_importance + type_category + in_use
    }

    pub fn upcase_table() -> Self {
        let type_code = type_code::TypeCode::UpcaseTable;
        let type_importance = false;
        let type_category = false;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }

    pub fn volume_guid() -> Self {
        let type_code = type_code::TypeCode::VolumeGuid;
        let type_importance = true;
        let type_category = false;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }

    pub fn volume_label() -> Self {
        let type_code = type_code::TypeCode::VolumeLabel;
        let type_importance = false;
        let type_category = false;
        let in_use = true;
        Self {
            type_code,
            type_importance,
            type_category,
            in_use,
        }
    }
}

