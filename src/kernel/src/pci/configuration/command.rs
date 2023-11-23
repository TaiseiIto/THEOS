// PCI Express Base Specification Revision 5.0 Version 1.0 7.5.1.1.3 Command Register
#[derive(Debug)]
pub struct Command {
    io_space_enable: bool,
    memory_space_enable: bool,
    bus_master_enable: bool,
    special_cycle_enable: bool,
    memory_write_and_invalidate: bool,
    vda_palette_snoop: bool,
    parity_error_responce: bool,
    idsel_stepping_wait_cycle_control: bool,
    serr_enable: bool,
    fast_back_to_back_transactions_enable: bool,
    interrupt_disable: bool,
}

impl Command {
    const IO_SPACE_ENABLE_SHIFT: usize = 0;
    const MEMORY_SPACE_ENABLE_SHIFT: usize = Self::IO_SPACE_ENABLE_SHIFT + 1;
    const BUS_MASTER_ENABLE_SHIFT: usize = Self::MEMORY_SPACE_ENABLE_SHIFT + 1;
    const SPECIAL_CYCLE_ENABLE_SHIFT: usize = Self::BUS_MASTER_ENABLE_SHIFT + 1;
    const MEMORY_WRITE_AND_INVALIDATE_SHIFT: usize = Self::SPECIAL_CYCLE_ENABLE_SHIFT + 1;
    const VGA_PALETTE_SNOOP_SHIFT: usize = Self::MEMORY_WRITE_AND_INVALIDATE_SHIFT + 1;
    const PARITY_ERROR_RESPONCE_SHIFT: usize = Self::VGA_PALETTE_SNOOP_SHIFT;
    const IDSEL_STEPPING_WAIT_CYCLE_CONTROL_SHIFT: usize = Self::PARITY_ERROR_RESPONCE_SHIFT + 1;
    const SERR_ENABLE_SHIFT: usize = Self::IDSEL_STEPPING_WAIT_CYCLE_CONTROL_SHIFT + 1;
    const FAST_BACK_TO_BACK_TRANSACTIONS_ENABLE_SHIFT: usize = Self::SERR_ENABLE_SHIFT + 1;
    const INTERRUPT_DISABLE_SHIFT: usize = Self::FAST_BACK_TO_BACK_TRANSACTIONS_ENABLE_SHIFT + 1;

    const IO_SPACE_ENABLE_MASK: u16 = 1 << Self::IO_SPACE_ENABLE_SHIFT;
    const MEMORY_SPACE_ENABLE_MASK: u16 = 1 << Self::MEMORY_SPACE_ENABLE_SHIFT;
    const BUS_MASTER_ENABLE_MASK: u16 = 1 << Self::BUS_MASTER_ENABLE_SHIFT;
    const SPECIAL_CYCLE_ENABLE_MASK: u16 = 1 << Self::SPECIAL_CYCLE_ENABLE_SHIFT;
    const MEMORY_WRITE_AND_INVALIDATE_MASK: u16 = 1 << Self::MEMORY_WRITE_AND_INVALIDATE_SHIFT;
    const VGA_PALETTE_SNOOP_MASK: u16 = 1 << Self::VGA_PALETTE_SNOOP_SHIFT;
    const PARITY_ERROR_RESPONCE_MASK: u16 = 1 << Self::PARITY_ERROR_RESPONCE_SHIFT;
    const IDSEL_STEPPING_WAIT_CYCLE_CONTROL_MASK: u16 = 1 << Self::IDSEL_STEPPING_WAIT_CYCLE_CONTROL_SHIFT;
    const SERR_ENABLE_MASK: u16 = 1 << Self::SERR_ENABLE_SHIFT;
    const FAST_BACK_TO_BACK_TRANSACTIONS_ENABLE_MASK: u16 = 1 << Self::FAST_BACK_TO_BACK_TRANSACTIONS_ENABLE_SHIFT;
    const INTERRUPT_DISABLE_MASK: u16 = 1 << Self::INTERRUPT_DISABLE_SHIFT;
}

impl From<u16> for Command {
    fn from(command: u16) -> Self {
        let io_space_enable: bool = command & Self::IO_SPACE_ENABLE_MASK != 0;
        let memory_space_enable: bool = command & Self::MEMORY_SPACE_ENABLE_MASK != 0;
        let bus_master_enable: bool = command & Self::BUS_MASTER_ENABLE_MASK != 0;
        let special_cycle_enable: bool = command & Self::SPECIAL_CYCLE_ENABLE_MASK != 0;
        let memory_write_and_invalidate: bool = command & Self::MEMORY_WRITE_AND_INVALIDATE_MASK != 0;
        let vda_palette_snoop: bool = command & Self::VGA_PALETTE_SNOOP_MASK != 0;
        let parity_error_responce: bool = command & Self::PARITY_ERROR_RESPONCE_MASK != 0;
        let idsel_stepping_wait_cycle_control: bool = command & Self::IDSEL_STEPPING_WAIT_CYCLE_CONTROL_MASK != 0;
        let serr_enable: bool = command & Self::SERR_ENABLE_MASK != 0;
        let fast_back_to_back_transactions_enable: bool = command & Self::FAST_BACK_TO_BACK_TRANSACTIONS_ENABLE_MASK != 0;
        let interrupt_disable: bool = command & Self::INTERRUPT_DISABLE_MASK != 0;
        Self {
            io_space_enable,
            memory_space_enable,
            bus_master_enable,
            special_cycle_enable,
            memory_write_and_invalidate,
            vda_palette_snoop,
            parity_error_responce,
            idsel_stepping_wait_cycle_control,
            serr_enable,
            fast_back_to_back_transactions_enable,
            interrupt_disable,
        }
    }
}

