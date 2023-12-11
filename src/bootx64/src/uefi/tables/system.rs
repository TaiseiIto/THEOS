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
            services::boot::memory_allocation,
            types::{
                char16,
                handle,
            },
        },
    },
};

#[macro_export]
macro_rules! uefi_print {
    ($($arg:tt)*) => ($crate::uefi::tables::system::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! uefi_println {
    ($fmt:expr) => (uefi_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (uefi_print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    system()
        .write_fmt(args)
        .expect("Can't output to the screen!");
}

pub fn exit_boot_services<'a>() -> memory_allocation::Map<'a> {
        let memory_map = memory_allocation::Map::new();
        let memory_map_key: usize = memory_map.key();
        let image: handle::Handle = image();
        system()
            .boot_services
            .exit_boot_services(image, memory_map_key)
            .expect("Can't exit boot services!");
        memory_map
}

static mut SYSTEM: Option<&'static mut System<'static>> = None;
static mut IMAGE: Option<handle::Handle<'static>> = None;

pub fn system() -> &'static mut System<'static> {
    unsafe {
        (&mut SYSTEM).as_mut().expect("Can't get a system table!")
    }
}

pub fn image() -> handle::Handle<'static> {
    unsafe {
        (&IMAGE).as_ref().expect("Can't get a image handle!")
    }
}

pub fn init_system(image: handle::Handle<'static>, system: &'static mut System<'static>) {
    system.con_out.reset(false).expect("Can't initialize a system table!");
    unsafe {
        SYSTEM = Some(system);
        IMAGE = Some(image);
    }
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
    pub con_in: &'a simple_text_input::SimpleTextInput<'a>,
    console_out_handle: handle::Handle<'a>,
    pub con_out: &'a simple_text_output::SimpleTextOutput<'a>,
    standard_error_handle: handle::Handle<'a>,
    pub std_err: &'a simple_text_output::SimpleTextOutput<'a>,
    pub runtime_services: &'a runtime_services::RuntimeServices,
    pub boot_services: &'a boot_services::BootServices<'a>,
    number_of_table_entries: usize,
    configuration_table: &'a configuration::Configuration<'a>,
}

impl fmt::Debug for System<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let configuration_tables: configuration::Configurations = self.into();
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
            .field("configuration_tables", &configuration_tables)
            .finish()
    }
}

impl<'a> Into<configuration::Configurations<'a>> for &System<'a> {
    fn into(self) -> configuration::Configurations<'a> {
        configuration::Configurations::<'a>::new(self.configuration_table, self.number_of_table_entries)
    }
}

impl Write for System<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.con_out.print(s).map_err(|_| fmt::Error)
    }
}

