use {
    core::fmt,
    super::super::types::{
        char16,
        status,
    },
};

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 12.4 Simple Text Output Protocol
#[repr(C)]
pub struct SimpleTextOutput {
    reset: TextReset,
    output_string: TextString,
    test_string: TextTestString,
    query_mode: TextQueryMode,
}

impl fmt::Debug for SimpleTextOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reset: usize = self.reset as usize;
        let output_string: usize = self.output_string as usize;
        let test_string: usize = self.test_string as usize;
        let query_mode: usize = self.query_mode as usize;
        write!(f, "SimpleTextOutput {{\n").expect("Can't print a simple text output protocol!");
        write!(f, "    reset: {:#x},\n", reset).expect("Can't print a simple text output protocol!");
        write!(f, "    output_string: {:#x},\n", output_string).expect("Can't print a simple text output protocol!");
        write!(f, "    test_string: {:#x},\n", test_string).expect("Can't print a simple text test protocol!");
        write!(f, "    query_mode: {:#x},\n", query_mode).expect("Can't print a simple text output protocol!");
        write!(f, "}}")
    }
}

type TextReset = extern "efiapi" fn(&SimpleTextOutput, bool) -> status::Status;
type TextString = extern "efiapi" fn(&SimpleTextOutput, char16::String) -> status::Status;
type TextTestString = extern "efiapi" fn(&SimpleTextOutput, char16::String) -> status::Status;
type TextQueryMode = extern "efiapi" fn(&SimpleTextOutput, u64, &mut u64, &mut u64) -> status::Status;

