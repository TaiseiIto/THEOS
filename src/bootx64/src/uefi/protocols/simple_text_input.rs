use {
    core::fmt,
    super::super::types::status,
};

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 12.3 Simple Text Input Protocol
pub struct SimpleTextInput {
    reset: InputReset,
    read_key_stroke: InputReadKey,
}

impl fmt::Debug for SimpleTextInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reset: usize = self.reset as usize;
        let read_key_stroke: usize = self.read_key_stroke as usize;
        write!(f, "SimpleTextInput {{\n").expect("Can't print a simple text input protocol!");
        write!(f, "    reset: {:#x}\n", reset).expect("Can't print a simple text input protocol!");
        write!(f, "    read_key_stroke: {:#x}\n", read_key_stroke).expect("Can't print a simple text input protocol!");
        write!(f, "}}")
    }
}

type InputReset = fn(&SimpleTextInput, bool) -> status::Status;
type InputReadKey = fn(&SimpleTextInput, &mut InputKey) -> status::Status;

struct InputKey {
    scan_code: u16,
    unicode_char: u16,
}

