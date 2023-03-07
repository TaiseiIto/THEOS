// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 8.2 Variable Services

use {
    core::fmt,
    super::super::{
        boot::protocol_handler,
        super::types::{
            char16,
            status,
            void,
        },
    },
};

#[repr(C)]
pub struct GetVariable(extern "efiapi" fn(char16::String, &protocol_handler::Guid, &mut u32, &mut usize, &mut void::Void) -> status::Status);

impl fmt::Debug for GetVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct GetNextVariableName(extern "efiapi" fn(&mut usize, char16::MutString, &mut protocol_handler::Guid) -> status::Status);

impl fmt::Debug for GetNextVariableName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct SetVariable(extern "efiapi" fn(char16::String, &protocol_handler::Guid, &mut u32, &mut usize, &mut void::Void) -> status::Status);

impl fmt::Debug for SetVariable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct QueryVariableInfo(extern "efiapi" fn(u32, &mut u64, &mut u64, &mut u64) -> status::Status);

impl fmt::Debug for QueryVariableInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

