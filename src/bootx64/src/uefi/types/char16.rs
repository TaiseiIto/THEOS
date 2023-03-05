use core::{
    char,
    fmt,
};

// Reference
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 2.3.1 Data Types
// CHAR16 *
#[derive(Clone)]
#[repr(C)]
pub struct String(*const u16);

impl Iterator for String {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = unsafe {
            self.0.add(1)
        };
        let character: u16 = unsafe {
            *self.0
        };
        match character {
            0x0000 => None,
            _ => Some(character),
        }
    }
}

impl fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char16 = self.clone();
        write!(f, "\"").expect("Can't print an UTF16 string!");
        for character in char::decode_utf16(char16.into_iter()).filter_map(|character| character.ok()) {
            write!(f, "{}", character).expect("Can't print an UTF16 string!");
        }
        write!(f, "\"")
    }
}

