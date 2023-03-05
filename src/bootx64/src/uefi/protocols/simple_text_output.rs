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

struct TextReset(extern "efiapi" fn(&SimpleTextOutput, bool) -> status::Status);

impl fmt::Debug for TextReset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function = self.0 as usize;
        write!(f, "{:#x}", function)
    }
}

struct TextString(extern "efiapi" fn(&SimpleTextOutput, char16::String) -> status::Status);

impl fmt::Debug for TextString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function = self.0 as usize;
        write!(f, "{:#x}", function)
    }
}

struct TextTestString(extern "efiapi" fn(&SimpleTextOutput, char16::String) -> status::Status);

impl fmt::Debug for TextTestString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function = self.0 as usize;
        write!(f, "{:#x}", function)
    }
}

struct TextQueryMode(extern "efiapi" fn(&SimpleTextOutput, u64, &mut u64, &mut u64) -> status::Status);

impl fmt::Debug for TextQueryMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function = self.0 as usize;
        write!(f, "{:#x}", function)
    }
}

struct TextSetMode(extern "efiapi" fn(&SimpleTextOutput, u64) -> status::Status);

impl fmt::Debug for TextSetMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function = self.0 as usize;
        write!(f, "{:#x}", function)
    }
}

struct TextSetAttribute(extern "efiapi" fn(&SimpleTextOutput, u64) -> status::Status);

impl fmt::Debug for TextSetAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function = self.0 as usize;
        write!(f, "{:#x}", function)
    }
}

struct TextClearScreen(extern "efiapi" fn(&SimpleTextOutput) -> status::Status);

impl fmt::Debug for TextClearScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function = self.0 as usize;
        write!(f, "{:#x}", function)
    }
}

struct TextSetCursorPosition(extern "efiapi" fn(&SimpleTextOutput, u64, u64) -> status::Status);

impl fmt::Debug for TextSetCursorPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function = self.0 as usize;
        write!(f, "{:#x}", function)
    }
}

struct TextEnableCursor(extern "efiapi" fn(&SimpleTextOutput, bool) -> status::Status);

impl fmt::Debug for TextEnableCursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let function = self.0 as usize;
        write!(f, "{:#x}", function)
    }
}

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

