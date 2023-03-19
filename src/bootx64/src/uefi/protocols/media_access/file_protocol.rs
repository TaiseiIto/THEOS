// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 13.5 File Protocol

use {
    alloc::vec::Vec,
    super::super::super::{
        services::boot::protocol_handler,
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

impl FileProtocol {
    pub fn read(&self) -> Vec<u8> {
        let mut buffer = void::Void::new();
        let mut buffer_size: usize = 0;
        self.read.0(
            &self,
            &mut buffer_size,
            &mut buffer,
        );
        let mut buffer = allocator::Allocated::new(buffer_size, 1);
        let buffer: &mut [u8] = buffer.get_mut();
        {
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
        }
        buffer.to_vec()
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

