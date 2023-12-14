// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.4 Host Controller Operational Registers
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    usbcmd: u32,
    usbsts: u32,
    pagesize: u32,
    rsvd0: u64,
    dnctrl: u32,
    crcr: u32,
    rsvd1: u128,
    dcbaap: u64,
    config: u32,
}

