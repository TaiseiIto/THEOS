// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 8.2 Variable Services

use {
    super::super::{
        boot::protocol_handler,
        super::types::{
            char16,
            status,
            void,
        },
    },
    wrapped_function::WrappedFunction,
};

#[derive(WrappedFunction)]
#[repr(C)]
pub struct GetVariable(pub extern "efiapi" fn(char16::String, &protocol_handler::Guid, &mut u32, &mut usize, &mut void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct GetNextVariableName(pub extern "efiapi" fn(&mut usize, char16::MutString, &mut protocol_handler::Guid) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct SetVariable(pub extern "efiapi" fn(char16::String, &protocol_handler::Guid, &mut u32, &mut usize, &mut void::Void) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct QueryVariableInfo(pub extern "efiapi" fn(u32, &mut u64, &mut u64, &mut u64) -> status::Status);

