extern crate alloc;

use alloc::vec::Vec;

// PCI Express Specification Revision 5.0 Version 1.0 7.5.1.1.4 Status Register
#[allow(dead_code)]
#[derive(Debug)]
pub struct Register {
    immediate_readiness: bool,
    interrupt_status: bool,
    capabilities_list: bool,
    flag_66mhz_capable: bool,
    fast_back_to_back_transactions_capable: bool,
    master_data_parity_error: bool,
    devsel_timing: [bool; 2],
    signaled_target_abort: bool,
    received_target_abort: bool,
    received_master_abort: bool,
    signaled_system_error: bool,
    detected_parity_error: bool,
}

impl Register {
    const IMMEDIATE_READINESS_SHIFT: usize = 0;
    const INTERRUPT_STATUS_SHIFT: usize = 3;
    const CAPABILITIES_LIST_SHIFT: usize = 4;
    const FLAG_66MHZ_CAPABLE_SHIFT: usize = 5;
    const FAST_BACK_TO_BACK_TRANSACTIONS_CAPABLE_SHIFT: usize = 7;
    const MASTER_DATA_PARITY_ERROR_SHIFT: usize = 8;
    const DEVSEL_TIMING_SHIFT: [usize; 2] = [9, 10];
    const SIGNALED_TARGET_ABORT_SHIFT: usize = 11;
    const RECEIVED_TARGET_ABORT_SHIFT: usize = 12;
    const RECEIVED_MASTER_ABORT_SHIFT: usize = 13;
    const SIGNALED_SYSTEM_ERROR_SHIFT: usize = 14;
    const DETECTED_PARITY_ERROR_SHIFT: usize = 15;

    const IMMEDIATE_READINESS_MASK: u16 = 1 << Self::IMMEDIATE_READINESS_SHIFT;
    const INTERRUPT_STATUS_MASK: u16 = 1 << Self::INTERRUPT_STATUS_SHIFT;
    const CAPABILITIES_LIST_MASK: u16 = 1 << Self::CAPABILITIES_LIST_SHIFT;
    const FLAG_66MHZ_CAPABLE_MASK: u16 = 1 << Self::FLAG_66MHZ_CAPABLE_SHIFT;
    const FAST_BACK_TO_BACK_TRANSACTIONS_CAPABLE_MASK: u16 = 1 << Self::FAST_BACK_TO_BACK_TRANSACTIONS_CAPABLE_SHIFT;
    const MASTER_DATA_PARITY_ERROR_MASK: u16 = 1 << Self::MASTER_DATA_PARITY_ERROR_SHIFT;
    const DEVSEL_TIMING_MASK: [u16; 2] = [1 << Self::DEVSEL_TIMING_SHIFT[0], 1 << Self::DEVSEL_TIMING_SHIFT[1]];
    const SIGNALED_TARGET_ABORT_MASK: u16 = 1 << Self::SIGNALED_TARGET_ABORT_SHIFT;
    const RECEIVED_TARGET_ABORT_MASK: u16 = 1 << Self::RECEIVED_TARGET_ABORT_SHIFT;
    const RECEIVED_MASTER_ABORT_MASK: u16 = 1 << Self::RECEIVED_MASTER_ABORT_SHIFT;
    const SIGNALED_SYSTEM_ERROR_MASK: u16 = 1 << Self::SIGNALED_SYSTEM_ERROR_SHIFT;
    const DETECTED_PARITY_ERROR_MASK: u16 = 1 << Self::DETECTED_PARITY_ERROR_SHIFT;
}

impl From<u16> for Register {
    fn from(register: u16) -> Self {
        let immediate_readiness: bool = register & Self::IMMEDIATE_READINESS_MASK != 0;
        let interrupt_status: bool = register & Self::INTERRUPT_STATUS_MASK != 0;
        let capabilities_list: bool = register & Self::CAPABILITIES_LIST_MASK != 0;
        let flag_66mhz_capable: bool = register & Self::FLAG_66MHZ_CAPABLE_MASK != 0;
        let fast_back_to_back_transactions_capable: bool = register & Self::FAST_BACK_TO_BACK_TRANSACTIONS_CAPABLE_MASK != 0;
        let master_data_parity_error: bool = register & Self::MASTER_DATA_PARITY_ERROR_MASK != 0;
        let devsel_timing: [bool; 2] = Self::DEVSEL_TIMING_MASK
            .iter()
            .map(|mask| register & mask != 0)
            .collect::<Vec<bool>>()
            .try_into()
            .expect("Can't get a PCI configuration register!");
        let signaled_target_abort: bool = register & Self::SIGNALED_TARGET_ABORT_MASK != 0;
        let received_target_abort: bool = register & Self::RECEIVED_TARGET_ABORT_MASK != 0;
        let received_master_abort: bool = register & Self::RECEIVED_MASTER_ABORT_MASK != 0;
        let signaled_system_error: bool = register & Self::SIGNALED_SYSTEM_ERROR_MASK != 0;
        let detected_parity_error: bool = register & Self::DETECTED_PARITY_ERROR_MASK != 0;
        Self {
            immediate_readiness,
            interrupt_status,
            capabilities_list,
            flag_66mhz_capable,
            fast_back_to_back_transactions_capable,
            master_data_parity_error,
            devsel_timing,
            signaled_target_abort,
            received_target_abort,
            received_master_abort,
            signaled_system_error,
            detected_parity_error,
        }
    }
}

