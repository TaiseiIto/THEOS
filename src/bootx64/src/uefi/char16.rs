use core::{
    char,
    fmt,
};

// Reference
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 2.3.1 Data Types
#[derive(Clone)]
pub struct Char16 {
    character: *const u16,
}

impl Iterator for Char16 {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        self.character = unsafe {
            self.character.add(1)
        };
        let character: u16 = unsafe {
            *self.character
        };
        match character {
            0x0000 => None,
            _ => Some(character),
        }
    }
}

impl fmt::Debug for Char16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char16 = self.clone();
        for character in char::decode_utf16(char16.into_iter()).filter_map(|character| character.ok()) {
            write!(f, "{}", character).expect("Can't print an UTF16 string!");
        }
        write!(f, "")
    }
}

