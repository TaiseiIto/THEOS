// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 12.4 Simple Text Output Protocol

extern crate alloc;

use {
    alloc::vec::Vec,
    super::super::super::types::{
        char16,
        status,
    },
    wrapped_function::WrappedFunction,
};

#[derive(Debug)]
#[repr(C)]
pub struct SimpleTextOutput<'a> {
    reset: TextReset,
    output_string: TextString,
    test_string: TextTestString,
    query_mode: TextQueryMode,
    set_mode: TextSetMode,
    set_attribute: TextSetAttribute,
    clear_screen: TextClearScreen,
    set_cursor_position: TextSetCursorPosition,
    enable_cursor: TextEnableCursor,
    mode: &'a SimpleTextOutputMode,
}

impl SimpleTextOutput<'_> {
    pub fn reset(&self, extended_verification: bool) -> Result<(), status::Status> {
        match self.reset.0(self, extended_verification) {
            status::SUCCESS => Ok(()),
            error => Err(error),
        }
    }

    pub fn print(&self, string: &str) -> Result<(), status::Status> {
        let mut string: Vec<u16> = string
            .replace("\n", "\r\n")
            .chars()
            .filter_map(|c| (c as u32).try_into().ok())
            .collect();
        string.push(0);
        let string = char16::String::new(&string[0]);
        match self.output_string.0(self, string) {
            status::SUCCESS => Ok(()),
            error => Err(error),
        }
    }
}

#[derive(WrappedFunction)]
#[repr(C)]
struct TextReset(pub extern "efiapi" fn(&SimpleTextOutput, bool) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct TextString(pub extern "efiapi" fn(&SimpleTextOutput, char16::String) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct TextTestString(pub extern "efiapi" fn(&SimpleTextOutput, char16::String) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct TextQueryMode(pub extern "efiapi" fn(&SimpleTextOutput, usize, &mut usize, &mut usize) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct TextSetMode(pub extern "efiapi" fn(&SimpleTextOutput, usize) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct TextSetAttribute(pub extern "efiapi" fn(&SimpleTextOutput, usize) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct TextClearScreen(pub extern "efiapi" fn(&SimpleTextOutput) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct TextSetCursorPosition(pub extern "efiapi" fn(&SimpleTextOutput, usize, usize) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct TextEnableCursor(pub extern "efiapi" fn(&SimpleTextOutput, bool) -> status::Status);

#[derive(Debug)]
#[repr(C)]
struct SimpleTextOutputMode {
    max_mode: i32,
    mode: i32,
    attribute: i32,
    cursor_column: i32,
    cursor_row: i32,
    cursor_visible: bool,
}

