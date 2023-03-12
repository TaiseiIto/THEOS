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
        types::{
            handle,
            status,
            void,
        },
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
    connect_controller: protocol_handler::ConnectController,
    disconnect_controller: protocol_handler::DisconnectController,
    open_protocol: protocol_handler::OpenProtocol,
    close_protocol: protocol_handler::CloseProtocol,
    open_protocol_information: protocol_handler::OpenProtocolInformation,
    protocols_per_handle: protocol_handler::ProtocolsPerHandle,
    locate_handle_buffer: protocol_handler::LocateHandleBuffer,
    locate_protocol: protocol_handler::LocateProtocol,
    install_multiple_protocol_interfaces: protocol_handler::InstallMultipleProtocolInterfaces,
    uninstall_multiple_protocol_interfaces: protocol_handler::UninstallMultipleProtocolInterfaces,
    calculate_crc32: boot::CalculateCrc32,
    copy_mem: boot::CopyMem,
    set_mem: boot::SetMem,
    create_event_ex: event::CreateEventEx,
}

impl BootServices<'_> {
    pub fn allocate_pages(
        &self,
        allocate_type: memory_allocation::AllocateType,
        memory_type: memory_allocation::MemoryType,
        pages: usize,
        memory: &mut memory_allocation::PhysicalAddress,
    ) -> status::Status {
        self.allocate_pages.0(
            allocate_type,
            memory_type,
            pages,
            memory,
        )
    }

    pub fn allocate_pool(
        &self,
        memory_type: memory_allocation::MemoryType,
        size: usize,
        buffer: &mut &void::Void,
    ) -> status::Status {
        self.allocate_pool.0(
            memory_type,
            size,
            buffer,
        )
    }

    pub fn exit_boot_services(
        &self,
        image_handle: handle::Handle,
        map_key: usize,
    ) -> status::Status {
        self.exit_boot_services.0(
            image_handle,
            map_key,
        )
    }

    pub fn free_pages(
        &self,
        memory: memory_allocation::PhysicalAddress,
        pages: usize,
    ) -> status::Status {
        self.free_pages.0(
            memory,
            pages,
        )
    }

    pub fn free_pool(
        &self,
        buffer: &void::Void,
    ) -> status::Status {
        self.free_pool.0(buffer)
    }

    pub fn get_memory_map(
        &self,
        memory_map_size: &mut usize,
        memory_map: &mut u8,
        map_key: &mut usize,
        descriptor_size: &mut usize,
        descriptor_version: &mut u32,
    ) -> status::Status {
        self.get_memory_map.0(
            memory_map_size,
            memory_map,
            map_key,
            descriptor_size,
            descriptor_version,
        )
    }
}

