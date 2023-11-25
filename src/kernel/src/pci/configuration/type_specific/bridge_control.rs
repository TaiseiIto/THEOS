// PCI Express Specification Revision 5.0 Version 1.0 7.5.1.3.13 Bridge Control Register
#[allow(dead_code)]
#[derive(Debug)]
pub struct Register {
    parity_error_responce_enable: bool,
    serr_enable: bool,
    isa_enable: bool,
    vga_enable: bool,
    vga_16bit_decode: bool,
    master_abort_mode: bool,
    secondary_bus_reset: bool,
    fast_back_to_back_transactions_enable: bool,
    primary_discard_timer: bool,
    secondary_discard_timer: bool,
    discard_timer_status: bool,
    discard_timer_serr_enable: bool,
}

impl Register {
    const PARITY_ERROR_RESPONCE_ENABLE_SHIFT: usize = 0;
    const SERR_ENABLE_SHIFT: usize = Self::PARITY_ERROR_RESPONCE_ENABLE_SHIFT + 1;
    const ISA_ENABLE_SHIFT: usize = Self::SERR_ENABLE_SHIFT + 1;
    const VGA_ENABLE_SHIFT: usize = Self::ISA_ENABLE_SHIFT + 1;
    const VGA_16BIT_DECODE_SHIFT: usize = Self::VGA_ENABLE_SHIFT + 1;
    const MASTER_ABORT_MODE_SHIFT: usize = Self::VGA_16BIT_DECODE_SHIFT + 1;
    const SECONDARY_BUS_RESET_SHIFT: usize = Self::MASTER_ABORT_MODE_SHIFT + 1;
    const FAST_BACK_TO_BACK_TRANSACTIONS_ENABLE_SHIFT: usize = Self::SECONDARY_BUS_RESET_SHIFT + 1;
    const PRIMARY_DISCARD_TIMER_SHIFT: usize = Self::FAST_BACK_TO_BACK_TRANSACTIONS_ENABLE_SHIFT + 1;
    const SECONDARY_DISCARD_TIMER_SHIFT: usize = Self::PRIMARY_DISCARD_TIMER_SHIFT + 1;
    const DISCARD_TIMER_STATUS_SHIFT: usize = Self::SECONDARY_DISCARD_TIMER_SHIFT + 1;
    const DISCARD_TIMER_SERR_ENABLE_SHIFT: usize = Self::DISCARD_TIMER_STATUS_SHIFT + 1;

    const PARITY_ERROR_RESPONCE_ENABLE_MASK: u16 = 1 << Self::PARITY_ERROR_RESPONCE_ENABLE_SHIFT;
    const SERR_ENABLE_MASK: u16 = 1 << Self::SERR_ENABLE_SHIFT;
    const ISA_ENABLE_MASK: u16 = 1 << Self::ISA_ENABLE_SHIFT;
    const VGA_ENABLE_MASK: u16 = 1 << Self::VGA_ENABLE_SHIFT;
    const VGA_16BIT_DECODE_MASK: u16 = 1 << Self::VGA_16BIT_DECODE_SHIFT;
    const MASTER_ABORT_MODE_MASK: u16 = 1 << Self::MASTER_ABORT_MODE_SHIFT;
    const SECONDARY_BUS_RESET_MASK: u16 = 1 << Self::SECONDARY_BUS_RESET_SHIFT;
    const FAST_BACK_TO_BACK_TRANSACTIONS_ENABLE_MASK: u16 = 1 << Self::FAST_BACK_TO_BACK_TRANSACTIONS_ENABLE_SHIFT;
    const PRIMARY_DISCARD_TIMER_MASK: u16 = 1 << Self::PRIMARY_DISCARD_TIMER_SHIFT;
    const SECONDARY_DISCARD_TIMER_MASK: u16 = 1 << Self::SECONDARY_DISCARD_TIMER_SHIFT;
    const DISCARD_TIMER_STATUS_MASK: u16 = 1 << Self::DISCARD_TIMER_STATUS_SHIFT;
    const DISCARD_TIMER_SERR_ENABLE_MASK: u16 = 1 << Self::DISCARD_TIMER_SERR_ENABLE_SHIFT;
}

impl From<u16> for Register {
    fn from(register: u16) -> Self {
        let parity_error_responce_enable: bool = register & Self::PARITY_ERROR_RESPONCE_ENABLE_MASK != 0;
        let serr_enable: bool = register & Self::SERR_ENABLE_MASK != 0;
        let isa_enable: bool = register & Self::ISA_ENABLE_MASK != 0;
        let vga_enable: bool = register & Self::VGA_ENABLE_MASK != 0;
        let vga_16bit_decode: bool = register & Self::VGA_16BIT_DECODE_MASK != 0;
        let master_abort_mode: bool = register & Self::MASTER_ABORT_MODE_MASK != 0;
        let secondary_bus_reset: bool = register & Self::SECONDARY_BUS_RESET_MASK != 0;
        let fast_back_to_back_transactions_enable: bool = register & Self::FAST_BACK_TO_BACK_TRANSACTIONS_ENABLE_MASK != 0;
        let primary_discard_timer: bool = register & Self::PRIMARY_DISCARD_TIMER_MASK != 0;
        let secondary_discard_timer: bool = register & Self::SECONDARY_DISCARD_TIMER_MASK != 0;
        let discard_timer_status: bool = register & Self::DISCARD_TIMER_STATUS_MASK != 0;
        let discard_timer_serr_enable: bool = register & Self::DISCARD_TIMER_SERR_ENABLE_MASK != 0;
        Self {
            parity_error_responce_enable,
            serr_enable,
            isa_enable,
            vga_enable,
            vga_16bit_decode,
            master_abort_mode,
            secondary_bus_reset,
            fast_back_to_back_transactions_enable,
            primary_discard_timer,
            secondary_discard_timer,
            discard_timer_status,
            discard_timer_serr_enable,
        }
    }
}

