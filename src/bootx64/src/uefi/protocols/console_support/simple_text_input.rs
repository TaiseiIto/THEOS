use {
    super::super::super::types::{
        event,
        status,
    },
    wrapped_function::WrappedFunction,
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

#[derive(WrappedFunction)]
#[repr(C)]
struct InputReset(pub extern "efiapi" fn(&SimpleTextInput, bool) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct InputReadKey(pub extern "efiapi" fn(&SimpleTextInput, &mut InputKey) -> status::Status);

#[repr(C)]
struct InputKey {
    scan_code: u16,
    unicode_char: u16,
}

