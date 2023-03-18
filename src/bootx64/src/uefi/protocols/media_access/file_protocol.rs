// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 13.5 File Protocol

use {
    super::super::super::{
        services::boot::protocol_handler,
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

