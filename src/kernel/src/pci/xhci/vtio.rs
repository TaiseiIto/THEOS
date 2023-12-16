// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.7 VTIO Registers
#[allow(dead_code)]
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    cap: u32,
    ca: u32,
    da: [u32; 8],
    reserved0: [u8; 8],
    ia: [u32; 32],
    reserved1: [u8; 0x50],
    ea: [u32; 255],
}

