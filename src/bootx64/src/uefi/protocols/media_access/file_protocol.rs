// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 13.5 File Protocol

use {
    alloc::string::String,
    core::fmt,
    super::super::super::{
        services::{
            boot::protocol_handler,
            runtime::time,
        },
        super::allocator,
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

impl<'a> Iterator for &'a FileProtocol {
    type Item = FileInformation<'a>;

    fn next(&mut self) -> Option<FileInformation<'a>> {
        let mut buffer = void::Void::new();
        let mut buffer_size: usize = 0;
        self.read.0(
            &self,
            &mut buffer_size,
            &mut buffer,
        );
        match buffer_size {
            0 => None,
            mut buffer_size => {
                let mut allocated = allocator::Allocated::new(buffer_size, 1);
                let buffer: &mut [u8] = allocated.get_mut();
                let buffer: &mut u8 = &mut buffer[0];
                let buffer: *mut u8 = &mut *buffer;
                let buffer: usize = buffer as usize;
                let buffer: *mut void::Void = buffer as *mut void::Void;
                let buffer: &mut void::Void = unsafe {
                    &mut *buffer
                };
                match self.read.0(
                    &self,
                    &mut buffer_size,
                    buffer,
                ) {
                    status::SUCCESS => (),
                    _ => panic!("Can't read a file protocol!"),
                }
                let buffer: *const void::Void = &*buffer;
                let buffer: usize = buffer as usize;
                let file_info: *const FileInfo = buffer as *const FileInfo;
                let file_info: &FileInfo = unsafe {
                    &*file_info
                };
                let file_name: String = char16::String::new(&(file_info.file_name)).into();
                Some(FileInformation {
                    allocated,
                    file_name,
                    file_info,
                })
            },
        }
    }
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

#[allow(dead_code)]
pub struct FileInformation<'a> {
    allocated: allocator::Allocated<'a>,
    file_name: String,
    file_info: &'a FileInfo,
}

impl FileInformation<'_> {
    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }
}

impl<'a> fmt::Debug for FileInformation<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FileInformation")
            .field("file_name", &self.file_name)
            .field("file_info", &self.file_info)
            .finish()
    }
}

