use super::{
    CpuidOutRegisters,
    eax0x00000000::Eax0x00000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x00000005 {
    eax: Eax,
    ebx: Ebx,
    edx: Edx,
    ecx: Ecx,
}

impl Eax0x00000005 {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        if 5 <= eax0x00000000.max_eax() {
            let CpuidOutRegisters {
                eax,
                ebx,
                edx,
                ecx,
            } = CpuidOutRegisters::cpuid(5);
            let eax: Eax = eax.into();
            let ebx: Ebx = ebx.into();
            let edx: Edx = edx.into();
            let ecx: Ecx = ecx.into();
            Some(Self {
                eax,
                ebx,
                edx,
                ecx,
            })
        } else {
            None
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax {
    smallest_monitor_line_size: u16,
}

impl From<u32> for Eax {
    fn from(eax: u32) -> Self {
        let smallest_monitor_line_size: u16 = eax as u16;
        Self {
            smallest_monitor_line_size,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ebx {
    largest_monitor_line_size: u16,
}

impl From<u32> for Ebx {
    fn from(ebx: u32) -> Self {
        let largest_monitor_line_size: u16 = ebx as u16;
        Self {
            largest_monitor_line_size,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Edx {
    number_of_sub_c_states: [u8; 8],
}

impl From<u32> for Edx {
    fn from(edx: u32) -> Self {
        let mut number_of_sub_c_states: [u8; 8] = [0; 8];
        for i in 0..8 {
            number_of_sub_c_states[i] = ((edx >> (4 * i)) & 0x0f) as u8;
        }
        Self {
            number_of_sub_c_states,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ecx {
    enumeration_of_monitor_mwait_extensions_suppored: bool,
    supports_treating_interrupts_as_break_event_for_mwait: bool,
}

impl Ecx {
    const ENUMERATION_OF_MONITOR_MWAIT_EXTENSIONS_SUPPORED_SHIFT: usize = 0;
    const SUPPORTS_TREATING_INTERRUPTS_AS_BREAK_EVENT_FOR_MWAIT_SHIFT: usize = 1;

    const ENUMERATION_OF_MONITOR_MWAIT_EXTENSIONS_SUPPORED_MASK: u32 = 1 << Self::ENUMERATION_OF_MONITOR_MWAIT_EXTENSIONS_SUPPORED_SHIFT;
    const SUPPORTS_TREATING_INTERRUPTS_AS_BREAK_EVENT_FOR_MWAIT_MASK: u32 = 1 << Self::SUPPORTS_TREATING_INTERRUPTS_AS_BREAK_EVENT_FOR_MWAIT_SHIFT;
}

impl From<u32> for Ecx {
    fn from(ecx: u32) -> Self {
        let enumeration_of_monitor_mwait_extensions_suppored = ecx & Self::ENUMERATION_OF_MONITOR_MWAIT_EXTENSIONS_SUPPORED_MASK != 0;
        let supports_treating_interrupts_as_break_event_for_mwait = ecx & Self::SUPPORTS_TREATING_INTERRUPTS_AS_BREAK_EVENT_FOR_MWAIT_MASK != 0;
        Self {
            enumeration_of_monitor_mwait_extensions_suppored,
            supports_treating_interrupts_as_break_event_for_mwait,
        }
    }
}

