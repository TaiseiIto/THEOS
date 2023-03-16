use super::{
    CpuidOutRegisters,
    eax0x00000000::Eax0x00000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x0000000a {
    eax: Eax,
    ebx: Ebx,
    edx: Edx,
}

impl Eax0x0000000a {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x0000000a;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x00000000.max_eax() {
            let CpuidOutRegisters {
                eax,
                ebx,
                edx,
                ecx: _,
            } = CpuidOutRegisters::cpuid(eax, ecx);
            let eax: Eax = eax.into();
            let ebx: Ebx = ebx.into();
            let edx: Edx = edx.into();
            Some(Self {
                eax,
                ebx,
                edx,
            })
        } else {
            None
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax {
    version_id: u8,
    general_purpose_performance_monitoring_counter: u8,
    bit_width_of_general_purpose_performance_monitoring_counter: u8,
    length_of_ebx_bit_vector: u8,
}

impl From<u32> for Eax {
    fn from(eax: u32) -> Self {
        let [
            version_id,
            general_purpose_performance_monitoring_counter,
            bit_width_of_general_purpose_performance_monitoring_counter,
            length_of_ebx_bit_vector,
        ]: [u8; 4] = eax.to_le_bytes();
        Self {
            version_id,
            general_purpose_performance_monitoring_counter,
            bit_width_of_general_purpose_performance_monitoring_counter,
            length_of_ebx_bit_vector,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ebx {
    core_cycle_event_not_available: bool,
    instruction_retired_event_not_available: bool,
    reference_cycles_event_not_available: bool,
    last_level_cache_reference_event_not_available: bool,
    last_level_cache_misses_event_not_available: bool,
    branch_instruction_retired_event_not_available: bool,
    branch_mispredict_retired_event_not_available: bool,
    top_down_slots_event_not_available: bool,
}

impl Ebx {
    const CORE_CYCLE_EVENT_NOT_AVAILABLE_SHIFT: usize = 0;
    const INSTRUCTION_RETIRED_EVENT_NOT_AVAILABLE_SHIFT: usize = 1;
    const REFERENCE_CYCLES_EVENT_NOT_AVAILABLE_SHIFT: usize = 2;
    const LAST_LEVEL_CACHE_REFERENCE_EVENT_NOT_AVAILABLE_SHIFT: usize = 3;
    const LAST_LEVEL_CACHE_MISSES_EVENT_NOT_AVAILABLE_SHIFT: usize = 4;
    const BRANCH_INSTRUCTION_RETIRED_EVENT_NOT_AVAILABLE_SHIFT: usize = 5;
    const BRANCH_MISPREDICT_RETIRED_EVENT_NOT_AVAILABLE_SHIFT: usize = 6;
    const TOP_DOWN_SLOTS_EVENT_NOT_AVAILABLE_SHIFT: usize = 7;

    const CORE_CYCLE_EVENT_NOT_AVAILABLE_MASK: u32 = (1 << Self::CORE_CYCLE_EVENT_NOT_AVAILABLE_SHIFT) as u32;
    const INSTRUCTION_RETIRED_EVENT_NOT_AVAILABLE_MASK: u32 = (1 << Self::INSTRUCTION_RETIRED_EVENT_NOT_AVAILABLE_SHIFT) as u32;
    const REFERENCE_CYCLES_EVENT_NOT_AVAILABLE_MASK: u32 = (1 << Self::REFERENCE_CYCLES_EVENT_NOT_AVAILABLE_SHIFT) as u32;
    const LAST_LEVEL_CACHE_REFERENCE_EVENT_NOT_AVAILABLE_MASK: u32 = (1 << Self::LAST_LEVEL_CACHE_REFERENCE_EVENT_NOT_AVAILABLE_SHIFT) as u32;
    const LAST_LEVEL_CACHE_MISSES_EVENT_NOT_AVAILABLE_MASK: u32 = (1 << Self::LAST_LEVEL_CACHE_MISSES_EVENT_NOT_AVAILABLE_SHIFT) as u32;
    const BRANCH_INSTRUCTION_RETIRED_EVENT_NOT_AVAILABLE_MASK: u32 = (1 << Self::BRANCH_INSTRUCTION_RETIRED_EVENT_NOT_AVAILABLE_SHIFT) as u32;
    const BRANCH_MISPREDICT_RETIRED_EVENT_NOT_AVAILABLE_MASK: u32 = (1 << Self::BRANCH_MISPREDICT_RETIRED_EVENT_NOT_AVAILABLE_SHIFT) as u32;
    const TOP_DOWN_SLOTS_EVENT_NOT_AVAILABLE_MASK: u32 = (1 << Self::TOP_DOWN_SLOTS_EVENT_NOT_AVAILABLE_SHIFT) as u32;
}

impl From<u32> for Ebx {
    fn from(ebx: u32) -> Self {
        let core_cycle_event_not_available = ebx & Self::CORE_CYCLE_EVENT_NOT_AVAILABLE_MASK != 0;
        let instruction_retired_event_not_available = ebx & Self::INSTRUCTION_RETIRED_EVENT_NOT_AVAILABLE_MASK != 0;
        let reference_cycles_event_not_available = ebx & Self::REFERENCE_CYCLES_EVENT_NOT_AVAILABLE_MASK != 0;
        let last_level_cache_reference_event_not_available = ebx & Self::LAST_LEVEL_CACHE_REFERENCE_EVENT_NOT_AVAILABLE_MASK != 0;
        let last_level_cache_misses_event_not_available = ebx & Self::LAST_LEVEL_CACHE_MISSES_EVENT_NOT_AVAILABLE_MASK != 0;
        let branch_instruction_retired_event_not_available = ebx & Self::BRANCH_INSTRUCTION_RETIRED_EVENT_NOT_AVAILABLE_MASK != 0;
        let branch_mispredict_retired_event_not_available = ebx & Self::BRANCH_MISPREDICT_RETIRED_EVENT_NOT_AVAILABLE_MASK != 0;
        let top_down_slots_event_not_available = ebx & Self::TOP_DOWN_SLOTS_EVENT_NOT_AVAILABLE_MASK != 0;
        Self {
            core_cycle_event_not_available,
            instruction_retired_event_not_available,
            reference_cycles_event_not_available,
            last_level_cache_reference_event_not_available,
            last_level_cache_misses_event_not_available,
            branch_instruction_retired_event_not_available,
            branch_mispredict_retired_event_not_available,
            top_down_slots_event_not_available,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Edx {
    number_of_contiguous_fixed_function_performance_counters: u8,
    bit_width_of_fixed_function_performance_counters: u8,
    anythread_deprecation: bool,
}

impl Edx {
    const NUMBER_OF_CONTIGUOUS_FIXED_FUNCTION_PERFORMANCE_COUNTERS_SHIFT: usize = 0;
    const BIT_WIDTH_OF_FIXED_FUNCTION_PERFORMANCE_COUNTERS_SHIFT: usize = 5;
    const ANYTHREAD_DEPRECATION_SHIFT: usize = 15;

    const NUMBER_OF_CONTIGUOUS_FIXED_FUNCTION_PERFORMANCE_COUNTERS_SHIFT_END: usize = 4;
    const BIT_WIDTH_OF_FIXED_FUNCTION_PERFORMANCE_COUNTERS_SHIFT_END: usize = 12;
    const ANYTHREAD_DEPRECATION_SHIFT_END: usize = 15;

    const NUMBER_OF_CONTIGUOUS_FIXED_FUNCTION_PERFORMANCE_COUNTERS_LENGTH: usize = Self::NUMBER_OF_CONTIGUOUS_FIXED_FUNCTION_PERFORMANCE_COUNTERS_SHIFT_END - Self::NUMBER_OF_CONTIGUOUS_FIXED_FUNCTION_PERFORMANCE_COUNTERS_SHIFT + 1;
    const BIT_WIDTH_OF_FIXED_FUNCTION_PERFORMANCE_COUNTERS_LENGTH: usize = Self::BIT_WIDTH_OF_FIXED_FUNCTION_PERFORMANCE_COUNTERS_SHIFT_END - Self::BIT_WIDTH_OF_FIXED_FUNCTION_PERFORMANCE_COUNTERS_SHIFT + 1;
    const ANYTHREAD_DEPRECATION_LENGTH: usize = Self::ANYTHREAD_DEPRECATION_SHIFT_END - Self::ANYTHREAD_DEPRECATION_SHIFT + 1;

    const NUMBER_OF_CONTIGUOUS_FIXED_FUNCTION_PERFORMANCE_COUNTERS_MASK: u32 = (((1 << Self::NUMBER_OF_CONTIGUOUS_FIXED_FUNCTION_PERFORMANCE_COUNTERS_LENGTH) - 1) << Self::NUMBER_OF_CONTIGUOUS_FIXED_FUNCTION_PERFORMANCE_COUNTERS_SHIFT) as u32;
    const BIT_WIDTH_OF_FIXED_FUNCTION_PERFORMANCE_COUNTERS_MASK: u32 = (((1 << Self::BIT_WIDTH_OF_FIXED_FUNCTION_PERFORMANCE_COUNTERS_LENGTH) - 1) << Self::BIT_WIDTH_OF_FIXED_FUNCTION_PERFORMANCE_COUNTERS_SHIFT) as u32;
    const ANYTHREAD_DEPRECATION_MASK: u32 = (((1 << Self::ANYTHREAD_DEPRECATION_LENGTH) - 1) << Self::ANYTHREAD_DEPRECATION_SHIFT) as u32;
}

impl From<u32> for Edx {
    fn from(edx: u32) -> Self {
        let number_of_contiguous_fixed_function_performance_counters = ((edx & Self::NUMBER_OF_CONTIGUOUS_FIXED_FUNCTION_PERFORMANCE_COUNTERS_MASK) >> Self::NUMBER_OF_CONTIGUOUS_FIXED_FUNCTION_PERFORMANCE_COUNTERS_SHIFT) as u8;
        let bit_width_of_fixed_function_performance_counters = ((edx & Self::BIT_WIDTH_OF_FIXED_FUNCTION_PERFORMANCE_COUNTERS_MASK) >> Self::BIT_WIDTH_OF_FIXED_FUNCTION_PERFORMANCE_COUNTERS_SHIFT) as u8;
        let anythread_deprecation = edx & Self::ANYTHREAD_DEPRECATION_MASK != 0;
        Self {
            number_of_contiguous_fixed_function_performance_counters,
            bit_width_of_fixed_function_performance_counters,
            anythread_deprecation,
        }
    }
}

