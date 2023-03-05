use {
    core::fmt,
    super::super::types::status,
};

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 12.3 Simple Text Input Protocol
pub struct SimpleTextInput {
    reset: InputReset,
}

impl fmt::Debug for SimpleTextInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reset: usize = self.reset as usize;
        write!(f, "SimpleTextInput {{\n").expect("Can't print a simple text input protocol!");
        write!(f, "    reset: {:#x}\n", reset).expect("Can't print a simple text input protocol!");
        write!(f, "}}")
    }
}

type InputReset = fn(&SimpleTextInput, bool) -> status::Status;

