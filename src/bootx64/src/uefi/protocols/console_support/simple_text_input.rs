use {
    core::fmt,
    super::super::super::types::{
        event,
        status,
    },
};

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 12.3 Simple Text Input Protocol
#[derive(Debug)]
#[repr(C)]
pub struct SimpleTextInput<'a> {
    reset: InputReset,
    read_key_stroke: InputReadKey,
    wait_for_key: event::Event<'a>,
}

#[repr(C)]
struct InputReset(extern "efiapi" fn(&SimpleTextInput, bool) -> status::Status);

impl fmt::Debug for InputReset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function = self.0 as usize;
        write!(f, "{:#x}", function)
    }
}

#[repr(C)]
struct InputReadKey(extern "efiapi" fn(&SimpleTextInput, &mut InputKey) -> status::Status);

impl fmt::Debug for InputReadKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function = self.0 as usize;
        write!(f, "{:#x}", function)
    }
}

#[repr(C)]
struct InputKey {
    scan_code: u16,
    unicode_char: u16,
}

