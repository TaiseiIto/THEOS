use {
    core::fmt,
    super::super::super::types::{
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

impl SimpleTextOutput<'_> {
    pub fn reset(&self, extended_verification: bool) -> status::Status {
        self.reset.call(self, extended_verification)
    }

    pub fn print(&self, string: &str) -> status::Status {
        for character in string.chars() {
            match self.put_char(character) {
                status::SUCCESS => (),
                error => return error,
            }
        }
        status::SUCCESS
    }

    fn put_char(&self, character: char) -> status::Status {
        if character == '\n' {
            match self.put_char('\r') {
                status::SUCCESS => (),
                error => return error,
            }
        }
        let mut buffer: [u16; 3] = [0x0000; 3];
        character.clone().encode_utf16(&mut buffer[..]);
        let string = char16::String::new(&buffer[0]);
        self.output_string.call(self, string)
    }
}

#[repr(C)]
struct TextReset(extern "efiapi" fn(&SimpleTextOutput, bool) -> status::Status);

impl TextReset {
    fn call(&self, this: &SimpleTextOutput, extended_verification: bool) -> status::Status {
        self.0(this, extended_verification)
    }
}

impl fmt::Debug for TextReset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
struct TextString(extern "efiapi" fn(&SimpleTextOutput, char16::String) -> status::Status);

impl TextString {
    fn call(&self, this: &SimpleTextOutput, string: char16::String) -> status::Status {
        self.0(this, string)
    }
}

impl fmt::Debug for TextString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
struct TextTestString(extern "efiapi" fn(&SimpleTextOutput, char16::String) -> status::Status);

impl fmt::Debug for TextTestString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
struct TextQueryMode(extern "efiapi" fn(&SimpleTextOutput, usize, &mut usize, &mut usize) -> status::Status);

impl fmt::Debug for TextQueryMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
struct TextSetMode(extern "efiapi" fn(&SimpleTextOutput, usize) -> status::Status);

impl fmt::Debug for TextSetMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
struct TextSetAttribute(extern "efiapi" fn(&SimpleTextOutput, usize) -> status::Status);

impl fmt::Debug for TextSetAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
struct TextClearScreen(extern "efiapi" fn(&SimpleTextOutput) -> status::Status);

impl fmt::Debug for TextClearScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
struct TextSetCursorPosition(extern "efiapi" fn(&SimpleTextOutput, usize, usize) -> status::Status);

impl fmt::Debug for TextSetCursorPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
struct TextEnableCursor(extern "efiapi" fn(&SimpleTextOutput, bool) -> status::Status);

impl fmt::Debug for TextEnableCursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
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

