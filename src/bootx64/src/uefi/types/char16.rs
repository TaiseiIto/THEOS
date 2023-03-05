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
pub struct String<'a>(&'a u16);

impl<'a> String<'a> {
    pub fn new(string: &'a u16) -> Self {
        Self(string)
    }
}

impl Iterator for String<'_> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let character: &u16 = self.0;
        let character = character as *const u16;
        let character = unsafe {
            character.add(1)
        };
        let character: &u16 = unsafe {
            &*character
        };
        self.0 = character;
        match *self.0 {
            0x0000 => None,
            _ => Some(*character),
        }
    }
}

impl fmt::Debug for String<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char16 = self.clone();
        write!(f, "\"").expect("Can't print an UTF16 string!");
        for character in char::decode_utf16(char16.into_iter()).filter_map(|character| character.ok()) {
            write!(f, "{}", character).expect("Can't print an UTF16 string!");
        }
        write!(f, "\"")
    }
}

