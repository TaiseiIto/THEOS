use {
    alloc::string,
    core::{
        char,
        fmt,
    },
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

impl Into<string::String> for String<'_> {
    fn into(self) -> string::String {
        self
            .map(|character| char::from_u32(character as u32).expect("Can't convert UTF-16LE into String!"))
            .collect()
    }
}

impl Iterator for String<'_> {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        match *self.0 {
            0x0000 => None,
            character => {
                let next_character: &u16 = self.0;
                let next_character = next_character as *const u16;
                let next_character = unsafe {
                    next_character.add(1)
                };
                let next_character: &u16 = unsafe {
                    &*next_character
                };
                self.0 = next_character;
                Some(character)
            },
        }
    }
}

impl fmt::Debug for String<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char16 = self.clone();
        write!(f, "\"").expect("Can't print an UTF16 string!");
        char::decode_utf16(char16.into_iter())
            .filter_map(|character| character.ok())
            .for_each(|character|write!(f, "{}", character).expect("Can't print an UTF16 string!"));
        write!(f, "\"")
    }
}

#[repr(C)]
pub struct MutString<'a>(&'a mut u16);

impl<'a> Into<String<'a>> for MutString<'a> {
    fn into(self) -> String<'a> {
        String::<'a>(self.0)
    }
}

