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
    set_mode: TextSetMode,
    set_attribute: TextSetAttribute,
    clear_screen: TextClearScreen,
    set_cursor_position: TextSetCursorPosition,
    enable_cursor: TextEnableCursor,
}

impl fmt::Debug for SimpleTextOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reset: usize = self.reset as usize;
        let output_string: usize = self.output_string as usize;
        let test_string: usize = self.test_string as usize;
        let query_mode: usize = self.query_mode as usize;
        let set_mode: usize = self.set_mode as usize;
        let set_attribute: usize = self.set_attribute as usize;
        let clear_screen: usize = self.clear_screen as usize;
        let set_cursor_position: usize = self.set_cursor_position as usize;
        let enable_cursor: usize = self.enable_cursor as usize;
        write!(f, "SimpleTextOutput {{\n").expect("Can't print a simple text output protocol!");
        write!(f, "    reset: {:#x},\n", reset).expect("Can't print a simple text output protocol!");
        write!(f, "    output_string: {:#x},\n", output_string).expect("Can't print a simple text output protocol!");
        write!(f, "    test_string: {:#x},\n", test_string).expect("Can't print a simple text test protocol!");
        write!(f, "    query_mode: {:#x},\n", query_mode).expect("Can't print a simple text output protocol!");
        write!(f, "    set_mode: {:#x},\n", set_mode).expect("Can't print a simple text output protocol!");
        write!(f, "    set_attribute: {:#x},\n", set_attribute).expect("Can't print a simple text output protocol!");
        write!(f, "    clear_screen: {:#x},\n", clear_screen).expect("Can't print a simple text output protocol!");
        write!(f, "    set_cursor_position: {:#x},\n", set_cursor_position).expect("Can't print a simple text output protocol!");
        write!(f, "    enable_cursor: {:#x},\n", enable_cursor).expect("Can't print a simple text output protocol!");
        write!(f, "}}")
    }
}

type TextReset = extern "efiapi" fn(&SimpleTextOutput, bool) -> status::Status;
type TextString = extern "efiapi" fn(&SimpleTextOutput, char16::String) -> status::Status;
type TextTestString = extern "efiapi" fn(&SimpleTextOutput, char16::String) -> status::Status;
type TextQueryMode = extern "efiapi" fn(&SimpleTextOutput, u64, &mut u64, &mut u64) -> status::Status;
type TextSetMode = extern "efiapi" fn(&SimpleTextOutput, u64) -> status::Status;
type TextSetAttribute = extern "efiapi" fn(&SimpleTextOutput, u64) -> status::Status;
type TextClearScreen = extern "efiapi" fn(&SimpleTextOutput) -> status::Status;
type TextSetCursorPosition = extern "efiapi" fn(&SimpleTextOutput, u64, u64) -> status::Status;
type TextEnableCursor = extern "efiapi" fn(&SimpleTextOutput, bool) -> status::Status;

