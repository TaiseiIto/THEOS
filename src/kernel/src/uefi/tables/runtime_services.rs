use super::{
    header,
    super::services::runtime::{
        self,
        time,
        variable,
        virtual_memory,
    },
};

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 4.5 Runtime Services Table
#[derive(Debug)]
#[repr(C)]
pub struct RuntimeServices {
    header: header::Header,
    get_time: time::GetTime,
    set_time: time::SetTime,
    get_wakeup_time: time::GetWakeupTime,
    set_wakeup_time: time::SetWakeupTime,
    set_virtual_address_map: virtual_memory::SetVirtualAddressMap,
    convert_pointer: virtual_memory::ConvertPointer,
    get_variable: variable::GetVariable,
    get_next_variable_name: variable::GetNextVariableName,
    set_variable: variable::SetVariable,
    get_next_high_monotonic_count: runtime::GetNextHighMonotonicCount,
    reset_system: runtime::ResetSystem,
    update_capsule: runtime::UpdateCapsule,
    query_capsule_capabilities: runtime::QueryCapsuleCapabilities,
    query_variable_info: variable::QueryVariableInfo,
}

