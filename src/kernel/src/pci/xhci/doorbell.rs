use core::fmt;

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.6 Doorbell Registers
#[derive(Debug)]
pub struct Registers(u32);

