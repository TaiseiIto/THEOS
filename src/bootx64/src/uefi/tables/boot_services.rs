use super::{
    header,
    super::{
        services::boot::{
            event,
            image,
            memory_allocation,
            protocol_handler,
            self,
        },
        types::void,
    },
};

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 4.4 Boot Services Table
#[derive(Debug)]
#[repr(C)]
pub struct BootServices<'a> {
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
    signal_event: event::SignalEvent,
    clone_event: event::CloseEvent,
    check_event: event::CheckEvent,
    install_protocol_interface: protocol_handler::InstallProtocolInterface,
    reinstall_protocol_interface: protocol_handler::ReinstallProtocolInterface,
    uninstall_protocol_interface: protocol_handler::UninstallProtocolInterface,
    handle_protocol: protocol_handler::HandleProtocol,
    reserved: &'a void::Void,
    register_protocol_notify: protocol_handler::RegisterProtocolNotify,
    locate_handle: protocol_handler::LocateHandle,
    locate_device_path: protocol_handler::LocateDevicePath,
    install_configuration_table: boot::InstallConfigurationTable,
    load_image: image::LoadImage,
    start_image: image::StartImage,
    exit: image::Exit,
    unload_image: image::UnloadImage,
    exit_boot_services: image::ExitBootServices,
    get_next_monotonic_count: boot::GetNextMonotonicCount,
    stall: boot::Stall,
    set_watchdoc_timer: boot::SetWatchdogTimer,
}

