use super::{
    header,
    super::services::boot::{
        event,
        memory_allocation,
    },
};

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 4.4 Boot Services Table
#[derive(Debug)]
#[repr(C)]
pub struct BootServices {
    header: header::Header,
    raise_tpl: event::RaiseTpl,
    restore_tpl: event::RestoreTpl,
    allocate_pages: memory_allocation::AllocatePages,
    free_pages: memory_allocation::FreePages,
    get_memory_map: memory_allocation::GetMemoryMap,
    allocate_pool: memory_allocation::AllocatePool,
    free_pool: memory_allocation::FreePool,
    create_event: event::CreateEvent,
    set_timer: event::SetTimer,
    wait_for_event: event::WaitForEvent,
}

