// Reference
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 2.3.1 Data Types
#[derive(Debug)]
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

