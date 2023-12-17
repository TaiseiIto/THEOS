use core::fmt;

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.7 VTIO Registers
#[allow(dead_code)]
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    cap: Cap,
    ca: Ca,
    da: Da,
    reserved0: [u8; 8],
    ia: [u32; 32],
    reserved1: [u8; 0x50],
    ea: [u32; 255],
}

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.7.1 VTIO Capability Register (VTIOCAP)
#[derive(Clone, Copy)]
struct Cap(u32);

impl Cap {
    const PDMAID_BEGIN: usize = 0;
    const PDMAID_LENGTH: usize = 16;
    const PDMAID_END: usize = Self::PDMAID_BEGIN + Self::PDMAID_LENGTH;
    const ADMAID_BEGIN: usize = Self::PDMAID_END;
    const ADMAID_LENGTH: usize = 16;
    const ADMAID_END: usize = Self::ADMAID_BEGIN + Self::ADMAID_LENGTH;

    const PDMAID_MASK: u32 = (1 << Self::PDMAID_END) - (1 << Self::PDMAID_BEGIN);
    const ADMAID_MASK: u32 = u32::MAX - (1 << Self::ADMAID_BEGIN) + 1;

    fn pdmaid(&self) -> u16 {
        ((self.0 & Self::PDMAID_MASK) >> Self::PDMAID_BEGIN) as u16
    }

    fn admaid(&self) -> u16 {
        ((self.0 & Self::ADMAID_MASK) >> Self::ADMAID_BEGIN) as u16
    }
}

impl fmt::Debug for Cap {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CAP")
            .field("self", &self.0)
            .field("PDMAID", &self.pdmaid())
            .field("ADMAID", &self.admaid())
            .finish()
    }
}

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.7.2 VTIO Common Assignment Register 1 (VTIOCA1)
#[derive(Clone, Copy)]
struct Ca(u32);

impl Ca {
    const CRDIDA_SHIFT: usize = 1;
    const DCBAADIDA_SHIFT: usize = 2;
    const SPBADIDA_SHIFT: usize = 3;
    const SPBDIDA_SHIFT: usize = 4;
    const ICDIDA_SHIFT: usize = 6;
    const MSIDIDA_SHIFT: usize = 7;
    const PBDIDA_SHIFT: usize = 8;
    const DCDIDA_SHIFT: usize = 9;
    const EPDIDA_SHIFT: usize = 10;

    const CRDIDA_MASK: u32 = 1 << Self::CRDIDA_SHIFT;
    const DCBAADIDA_MASK: u32 = 1 << Self::DCBAADIDA_SHIFT;
    const SPBADIDA_MASK: u32 = 1 << Self::SPBADIDA_SHIFT;
    const SPBDIDA_MASK: u32 = 1 << Self::SPBDIDA_SHIFT;
    const ICDIDA_MASK: u32 = 1 << Self::ICDIDA_SHIFT;
    const MSIDIDA_MASK: u32 = 1 << Self::MSIDIDA_SHIFT;
    const PBDIDA_MASK: u32 = 1 << Self::PBDIDA_SHIFT;
    const DCDIDA_MASK: u32 = 1 << Self::DCDIDA_SHIFT;
    const EPDIDA_MASK: u32 = 1 << Self::EPDIDA_SHIFT;

    fn crdida(&self) -> bool {
        self.0 & Self::CRDIDA_MASK != 0
    }

    fn dcbaadida(&self) -> bool {
        self.0 & Self::DCBAADIDA_MASK != 0
    }

    fn spbadida(&self) -> bool {
        self.0 & Self::SPBADIDA_MASK != 0
    }

    fn spbdida(&self) -> bool {
        self.0 & Self::SPBDIDA_MASK != 0
    }

    fn icdida(&self) -> bool {
        self.0 & Self::ICDIDA_MASK != 0
    }

    fn msidida(&self) -> bool {
        self.0 & Self::MSIDIDA_MASK != 0
    }

    fn pbdida(&self) -> bool {
        self.0 & Self::PBDIDA_MASK != 0
    }

    fn dcdida(&self) -> bool {
        self.0 & Self::DCDIDA_MASK != 0
    }

    fn epdida(&self) -> bool {
        self.0 & Self::EPDIDA_MASK != 0
    }
}

impl fmt::Debug for Ca {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CA")
            .field("self", &self.0)
            .field("CRDIDA", &self.crdida())
            .field("DCBAADIDA", &self.dcbaadida())
            .field("SPBADIDA", &self.spbadida())
            .field("SPBDIDA", &self.spbdida())
            .field("ICDIDA", &self.icdida())
            .field("MSIDIDA", &self.msidida())
            .field("PBDIDA", &self.pbdida())
            .field("DCDIDA", &self.dcdida())
            .field("EPDIDA", &self.epdida())
            .finish()
    }
}

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.7.3 VTIO Device Assignment Registers 1 to 8 (VTIODA{1..8})
#[derive(Clone, Copy)]
struct Da([u32; 8]);

impl Da {
    fn bit(&self, index: usize) -> bool {
        let (index, shift): (usize, usize) = (index / u32::BITS as usize, index % u32::BITS as usize);
        self.0[index] & (1 << shift) != 0
    }
}

impl fmt::Debug for Da {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_map()
            .entries((0..self.0.len() * u32::BITS as usize)
                .map(|index| (index, self.bit(index))))
            .finish()
    }
}

