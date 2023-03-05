use super::{
    header,
    super::{
        protocols::{
            simple_text_input,
            simple_text_output,
        },
        types::{
            char16,
            handle,
            status,
        },
    },
};

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 4.3 System Table
#[derive(Debug)]
#[repr(C)]
pub struct System<'a> {
    header: header::Header,
    firmware_vendor: char16::String<'a>,
    firmware_revision: u32,
    console_in_handle: handle::Handle<'a>,
    con_in: &'a simple_text_input::SimpleTextInput<'a>,
    console_out_handle: handle::Handle<'a>,
    con_out: &'a simple_text_output::SimpleTextOutput<'a>,
}

impl System<'_> {
    pub fn con_out_reset(&self, extended_verification: bool) -> status::Status {
        self.con_out.reset(extended_verification)
    }

    pub fn con_out_output_string(&self, string: &str) -> status::Status {
        self.con_out.output_string(string)
    }
}

