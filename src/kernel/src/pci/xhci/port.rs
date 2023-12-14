// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.4 Host Controller Operational Registers
// Table 5-19: Host Controller USB Port Register Set
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    portsc: u32,
    portpmsc: u32,
    portli: u32,
    porthlpmc: u32,
}

