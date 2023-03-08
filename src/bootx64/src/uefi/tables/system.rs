use {
    core::fmt::{
        self,
        Write,
    },
    super::{
        boot_services,
        configuration,
        header,
        runtime_services,
        super::{
            protocols::console_support::{
                simple_text_input,
                simple_text_output,
            },
            types::{
                char16,
                handle,
                status,
            },
        },
    },
};

#[macro_export]
macro_rules! uefi_print {
    ($system:expr, $($arg:tt)*) => ($crate::uefi::tables::system::print($system, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! uefi_println {
    ($system:expr, $fmt:expr) => (uefi_print!($system, concat!($fmt, "\n")));
    ($system:expr, $fmt:expr, $($arg:tt)*) => (uefi_print!($system, concat!($fmt, "\n"), $($arg)*));
}

pub fn print(system: &mut System<'_>, args: fmt::Arguments) {
    system.write_fmt(args).expect("Can't output to the screen!");
}

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 4.3 System Table
#[derive(Clone)]
#[repr(C)]
pub struct System<'a> {
    header: header::Header,
    firmware_vendor: char16::String<'a>,
    firmware_revision: u32,
    console_in_handle: handle::Handle<'a>,
    con_in: &'a simple_text_input::SimpleTextInput<'a>,
    console_out_handle: handle::Handle<'a>,
    pub con_out: &'a simple_text_output::SimpleTextOutput<'a>,
    standard_error_handle: handle::Handle<'a>,
    std_err: &'a simple_text_output::SimpleTextOutput<'a>,
    runtime_services: &'a runtime_services::RuntimeServices,
    boot_services: &'a boot_services::BootServices<'a>,
    number_of_table_entries: usize,
    configuration_table: &'a configuration::Configuration<'a>,
}

impl fmt::Debug for System<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_struct("System")
            .field("header", &self.header)
            .field("firmware_vendor", &self.firmware_vendor)
            .field("firmware_revision", &self.firmware_revision)
            .field("console_in_handle", &self.console_in_handle)
            .field("con_in", &self.con_in)
            .field("console_out_handle", &self.console_out_handle)
            .field("con_out", &self.con_out)
            .field("standard_error_handle", &self.standard_error_handle)
            .field("std_err", &self.std_err)
            .field("runtime_services", &self.runtime_services)
            .field("boot_services", &self.boot_services)
            .field("number_of_table_entries", &self.number_of_table_entries)
            .field("configuration_table", &self.configuration_table)
            .finish()
    }
}

impl Write for System<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.con_out.print(s) {
            status::SUCCESS => Ok(()),
            _ => Err(fmt::Error),
        }
    }
}

