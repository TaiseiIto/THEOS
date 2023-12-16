// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Registers {
    iman: u32,
    imod: u32,
    erstsz: u32,
    reserved: u32,
    erstba: u64,
    erdp: u64,
}

