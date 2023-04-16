// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 13.5 File Protocol

use {
    super::super::super::{
        services::{
            boot::protocol_handler,
            runtime::time,
        },
        types::{
            char16,
            event,
            status,
            void,
        },
    },
    wrapped_function::WrappedFunction,
};

#[derive(Debug)]
#[repr(C)]
pub struct FileProtocol {
    revision: u64,
    open: FileOpen,
    close: FileClose,
    delete: FileDelete,
    read: FileRead,
    write: FileWrite,
    get_position: FileGetPosition,
    set_position: FileSetPosition,
    get_info: FileGetInfo,
    set_info: FileSetInfo,
    flush: FileFlush,
    open_ex: FileOpenEx,
    read_ex: FileReadEx,
    write_ex: FileWriteEx,
    flush_ex: FileFlushEx,
}

impl Drop for FileProtocol {
    fn drop(&mut self) {
        match self.close.0(&self) {
            status::SUCCESS => (),
            _ => panic!("Can't close a file protocol!"),
        }
    }
}

#[derive(WrappedFunction)]
#[repr(C)]
struct FileOpen(pub extern "efiapi" fn(&FileProtocol, &mut &FileProtocol, char16::String, u64, u64) -> status::Status);

pub struct OpenMode {
    read: bool,
    write: bool,
    create: bool,
}

impl OpenMode {
    pub fn new(
        read: bool,
        write: bool,
        create: bool,
    ) -> Self {
        Self {
            read,
            write,
            create,
        }
    }
}

impl Into<u64> for &OpenMode {
    fn into(self) -> u64 {
        let read: u64 = match self.read {
            true => 0x0000000000000001,
            false => 0x0000000000000000,
        };
        let write: u64 = match self.write {
            true => 0x0000000000000002,
            false => 0x0000000000000000,
        };
        let create: u64 = match self.create {
            true => 0x8000000000000000,
            false => 0x0000000000000000,
        };
        read | write | create
    }
}

pub struct Attributes {
    read_only: bool,
    hidden: bool,
    system: bool,
    reserved: bool,
    directory: bool,
    archive: bool,
}

impl Attributes {
    pub fn new(
        read_only: bool,
        hidden: bool,
        system: bool,
        reserved: bool,
        directory: bool,
        archive: bool,
    ) -> Self {
        Self {
            read_only,
            hidden,
            system,
            reserved,
            directory,
            archive,
        }
    }
}

impl Into<u64> for &Attributes {
    fn into(self) -> u64 {
        let read_only: u64 = match self.read_only {
            true => 0x0000000000000001,
            false => 0x0000000000000000,
        };
        let hidden: u64 = match self.hidden {
            true => 0x0000000000000002,
            false => 0x0000000000000000,
        };
        let system: u64 = match self.system {
            true => 0x0000000000000004,
            false => 0x0000000000000000,
        };
        let reserved: u64 = match self.reserved {
            true => 0x0000000000000008,
            false => 0x0000000000000000,
        };
        let directory: u64 = match self.directory {
            true => 0x0000000000000010,
            false => 0x0000000000000000,
        };
        let archive: u64 = match self.archive {
            true => 0x0000000000000020,
            false => 0x0000000000000000,
        };
        read_only | hidden | system | reserved | directory | archive
    }
}

#[derive(WrappedFunction)]
#[repr(C)]
struct FileClose(pub extern "efiapi" fn(&FileProtocol) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct FileDelete(pub extern "efiapi" fn(&FileProtocol) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct FileRead(pub extern "efiapi" fn(&FileProtocol, &mut usize, &mut void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct FileWrite(pub extern "efiapi" fn(&FileProtocol, &usize, &void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct FileOpenEx(pub extern "efiapi" fn(&FileProtocol, &mut &FileProtocol, char16::String, u64, u64, &mut FileIoToken) -> status::Status);

#[derive(Debug)]
#[repr(C)]
pub struct FileIoToken<'a> {
    event: event::Event<'a>,
    status: status::Status,
    buffer_size: usize,
    buffer: &'a void::Void,
}

#[derive(WrappedFunction)]
#[repr(C)]
struct FileReadEx(pub extern "efiapi" fn(&FileProtocol, &mut FileIoToken) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct FileWriteEx(pub extern "efiapi" fn(&FileProtocol, &mut FileIoToken) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct FileFlushEx(pub extern "efiapi" fn(&FileProtocol, &mut FileIoToken) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct FileSetPosition(pub extern "efiapi" fn(&FileProtocol, u64) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct FileGetPosition(pub extern "efiapi" fn(&FileProtocol, &mut u64) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct FileGetInfo(pub extern "efiapi" fn(&FileProtocol, &protocol_handler::Guid, &mut usize, &mut void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct FileSetInfo(pub extern "efiapi" fn(&FileProtocol, &protocol_handler::Guid, usize, &void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
struct FileFlush(pub extern "efiapi" fn(&FileProtocol) -> status::Status);

#[derive(Debug)]
#[repr(C)]
pub struct FileInfo {
    size: u64,
    file_size: u64,
    physical_size: u64,
    create_time: time::Time,
    last_access_time: time::Time,
    modification_time: time::Time,
    attribute: u64,
    file_name: u16,
}
