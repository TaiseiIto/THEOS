// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 10 Device Path Protocol

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 10.2 Device Path Protocol
#[repr(C)]
pub struct DevicePathProtocol {
    device_path_protocol_type: u8,
    device_path_protocol_subtype: u8,
    length: [u8; 2],
}

