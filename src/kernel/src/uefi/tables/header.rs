// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 4.2 EFI Table Header
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Header {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

