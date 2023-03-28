use {
    super::super::super::types::{
        char16,
        status,
    },
    wrapped_function::WrappedFunction,
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
    pub fn reset(&self, extended_verification: bool) -> Result<(), status::Status> {
        match self.reset.0(self, extended_verification) {
            status::SUCCESS => Ok(()),
            error => Err(error),
        }
    }

    pub fn print(&self, string: &str) -> Result<(), status::Status> {
        string
            .chars()
            .for_each(|character|
                self
                    .put_char(character)
                    .expect("Can't print a character!")
            );
        Ok(())
    }

    fn put_char(&self, character: char) -> Result<(), status::Status> {
        if character == '\n' {
            self.put_char('\r')?;
        }
        let mut buffer: [u16; 3] = [0x0000; 3];
        character.clone().encode_utf16(&mut buffer[..]);
        let string = char16::String::new(&buffer[0]);
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

