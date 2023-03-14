use super::{
    CpuidOutRegisters,
    eax0x00000000::Eax0x00000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x00000001 {
    eax: Eax,
    ebx: Ebx,
    edx: Edx,
}

impl Eax0x00000001 {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        if 1 <= eax0x00000000.max_eax() {
            let CpuidOutRegisters {
                eax,
                ebx,
                edx,
                ecx: _,
            } = CpuidOutRegisters::cpuid(1);
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
    stepping_id: u8,
    model: u8,
    family_id: u8,
    processor_type: u8,
    extended_model_id: u8,
    extended_family_id: u8,
}

impl Eax {
    const STEPPING_ID_SHIFT: usize = 0;
    const MODEL_SHIFT: usize = 4;
    const FAMILY_ID_SHIFT: usize = 8;
    const PROCESSOR_TYPE_SHIFT: usize = 12;
    const EXTENDED_MODEL_ID_SHIFT: usize = 16;
    const EXTENDED_FAMILY_ID_SHIFT: usize = 20;

    const STEPPING_ID_SHIFT_END: usize = 3;
    const MODEL_SHIFT_END: usize = 7;
    const FAMILY_ID_SHIFT_END: usize = 11;
    const PROCESSOR_TYPE_SHIFT_END: usize = 13;
    const EXTENDED_MODEL_ID_SHIFT_END: usize = 19;
    const EXTENDED_FAMILY_ID_SHIFT_END: usize = 27;

    const STEPPING_ID_LENGTH: usize = Self::STEPPING_ID_SHIFT_END - Self::STEPPING_ID_SHIFT + 1;
    const MODEL_LENGTH: usize = Self::MODEL_SHIFT_END - Self::MODEL_SHIFT + 1;
    const FAMILY_ID_LENGTH: usize = Self::FAMILY_ID_SHIFT_END - Self::FAMILY_ID_SHIFT + 1;
    const PROCESSOR_TYPE_LENGTH: usize = Self::PROCESSOR_TYPE_SHIFT_END - Self::PROCESSOR_TYPE_SHIFT + 1;
    const EXTENDED_MODEL_ID_LENGTH: usize = Self::EXTENDED_MODEL_ID_SHIFT_END - Self::EXTENDED_MODEL_ID_SHIFT + 1;
    const EXTENDED_FAMILY_ID_LENGTH: usize = Self::EXTENDED_FAMILY_ID_SHIFT_END - Self::EXTENDED_FAMILY_ID_SHIFT + 1;

    const STEPPING_ID_MASK: u32 = (((1 << Self::STEPPING_ID_LENGTH) - 1) << Self::STEPPING_ID_SHIFT) as u32;
    const MODEL_MASK: u32 = (((1 << Self::MODEL_LENGTH) - 1) << Self::MODEL_SHIFT) as u32;
    const FAMILY_ID_MASK: u32 = (((1 << Self::FAMILY_ID_LENGTH) - 1) << Self::FAMILY_ID_SHIFT) as u32;
    const PROCESSOR_TYPE_MASK: u32 = (((1 << Self::PROCESSOR_TYPE_LENGTH) - 1) << Self::PROCESSOR_TYPE_SHIFT) as u32;
    const EXTENDED_MODEL_ID_MASK: u32 = (((1 << Self::EXTENDED_MODEL_ID_LENGTH) - 1) << Self::EXTENDED_MODEL_ID_SHIFT) as u32;
    const EXTENDED_FAMILY_ID_MASK: u32 = (((1 << Self::EXTENDED_FAMILY_ID_LENGTH) - 1) << Self::EXTENDED_FAMILY_ID_SHIFT) as u32;
}
    
impl From<u32> for Eax {
    fn from(eax: u32) -> Self {
        let stepping_id = ((eax & Self::STEPPING_ID_MASK) >> Self::STEPPING_ID_SHIFT) as u8;
        let model = ((eax & Self::MODEL_MASK) >> Self::MODEL_SHIFT) as u8;
        let family_id = ((eax & Self::FAMILY_ID_MASK) >> Self::FAMILY_ID_SHIFT) as u8;
        let processor_type = ((eax & Self::PROCESSOR_TYPE_MASK) >> Self::PROCESSOR_TYPE_SHIFT) as u8;
        let extended_model_id = ((eax & Self::EXTENDED_MODEL_ID_MASK) >> Self::EXTENDED_MODEL_ID_SHIFT) as u8;
        let extended_family_id = ((eax & Self::EXTENDED_FAMILY_ID_MASK) >> Self::EXTENDED_FAMILY_ID_SHIFT) as u8;
        Self {
            stepping_id,
            model,
            family_id,
            processor_type,
            extended_model_id,
            extended_family_id,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ebx {
    brand_index: u8,
    clflush_line_size: u8,
    max_num_of_ids: u8,
    initial_apic_id: u8,
}

impl Ebx {
    const BRAND_INDEX_SHIFT: usize = 0;
    const CLFLUSH_LINE_SIZE_SHIFT: usize = 8;
    const MAX_NUM_OF_IDS_SHIFT: usize = 16;
    const INITIAL_APIC_ID_SHIFT: usize = 24;

    const BRAND_INDEX_SHIFT_END: usize = 7;
    const CLFLUSH_LINE_SIZE_SHIFT_END: usize = 15;
    const MAX_NUM_OF_IDS_SHIFT_END: usize = 23;
    const INITIAL_APIC_ID_SHIFT_END: usize = 31;

    const BRAND_INDEX_LENGTH: usize = Self::BRAND_INDEX_SHIFT_END - Self::BRAND_INDEX_SHIFT + 1;
    const CLFLUSH_LINE_SIZE_LENGTH: usize = Self::CLFLUSH_LINE_SIZE_SHIFT_END - Self::CLFLUSH_LINE_SIZE_SHIFT + 1;
    const MAX_NUM_OF_IDS_LENGTH: usize = Self::MAX_NUM_OF_IDS_SHIFT_END - Self::MAX_NUM_OF_IDS_SHIFT + 1;
    const INITIAL_APIC_ID_LENGTH: usize = Self::INITIAL_APIC_ID_SHIFT_END - Self::INITIAL_APIC_ID_SHIFT + 1;

    const BRAND_INDEX_MASK: u32 = (((1 << Self::BRAND_INDEX_LENGTH) - 1) << Self::BRAND_INDEX_SHIFT) as u32;
    const CLFLUSH_LINE_SIZE_MASK: u32 = (((1 << Self::CLFLUSH_LINE_SIZE_LENGTH) - 1) << Self::CLFLUSH_LINE_SIZE_SHIFT) as u32;
    const MAX_NUM_OF_IDS_MASK: u32 = (((1 << Self::MAX_NUM_OF_IDS_LENGTH) - 1) << Self::MAX_NUM_OF_IDS_SHIFT) as u32;
    const INITIAL_APIC_ID_MASK: u32 = (((1 << Self::INITIAL_APIC_ID_LENGTH) - 1) << Self::INITIAL_APIC_ID_SHIFT) as u32;
}

impl From<u32> for Ebx {
    fn from(ebx: u32) -> Self {
        let brand_index = ((ebx & Self::BRAND_INDEX_MASK) >> Self::BRAND_INDEX_SHIFT) as u8;
        let clflush_line_size = ((ebx & Self::CLFLUSH_LINE_SIZE_MASK) >> Self::CLFLUSH_LINE_SIZE_SHIFT) as u8;
        let max_num_of_ids = ((ebx & Self::MAX_NUM_OF_IDS_MASK) >> Self::MAX_NUM_OF_IDS_SHIFT) as u8;
        let initial_apic_id = ((ebx & Self::INITIAL_APIC_ID_MASK) >> Self::INITIAL_APIC_ID_SHIFT) as u8;
        Self {
            brand_index,
            clflush_line_size,
            max_num_of_ids,
            initial_apic_id,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Edx {
    floating_point_unit: bool,
    virtual_8086_mode_enhancements: bool,
    debugging_extensions: bool,
    page_size_extension: bool,
    time_stamp_counter: bool,
    model_specific_registers: bool,
    physical_address_extension: bool,
    machine_check_exception: bool,
    cmpxchg8b_instruction: bool,
    apic: bool,
    sysenter_and_sysexit_instructions: bool,
    memory_type_range_registers: bool,
    page_global_bit: bool,
    machine_check_architecture: bool,
    conditional_move_instructions: bool,
    page_attribute_table: bool,
    page_size_extension_36bit: bool,
    processor_serial_number: bool,
    clflush_instruction: bool,
    debug_store: bool,
    thermal_monitor_and_software_controlled_clock_facilities: bool,
    intel_mmx_technology: bool,
    fxsave_and_fxrstore_instructions: bool,
    sse: bool,
    sse2: bool,
    self_snoop: bool,
    max_apic_ids_reserved_field_is_valid: bool,
    thermal_monitor: bool,
    pending_break_enable: bool,
}

impl Edx {
    const FLOATING_POINT_UNIT_SHIFT: usize = 0;
    const VIRTUAL_8086_MODE_ENHANCEMENTS_SHIFT: usize = 1;
    const DEBUGGING_EXTENSIONS_SHIFT: usize = 2;
    const PAGE_SIZE_EXTENSION_SHIFT: usize = 3;
    const TIME_STAMP_COUNTER_SHIFT: usize = 4;
    const MODEL_SPECIFIC_REGISTERS_SHIFT: usize = 5;
    const PHYSICAL_ADDRESS_EXTENSION_SHIFT: usize = 6;
    const MACHINE_CHECK_EXCEPTION_SHIFT: usize = 7;
    const CMPXCHG8B_INSTRUCTION_SHIFT: usize = 8;
    const APIC_SHIFT: usize = 9;
    const SYSENTER_AND_SYSEXIT_INSTRUCTIONS_SHIFT: usize = 11;
    const MEMORY_TYPE_RANGE_REGISTERS_SHIFT: usize = 12;
    const PAGE_GLOBAL_BIT_SHIFT: usize = 13;
    const MACHINE_CHECK_ARCHITECTURE_SHIFT: usize = 14;
    const CONDITIONAL_MOVE_INSTRUCTIONS_SHIFT: usize = 15;
    const PAGE_ATTRIBUTE_TABLE_SHIFT: usize = 16;
    const PAGE_SIZE_EXTENSION_36BIT_SHIFT: usize = 17;
    const PROCESSOR_SERIAL_NUMBER_SHIFT: usize = 18;
    const CLFLUSH_INSTRUCTION_SHIFT: usize = 19;
    const DEBUG_STORE_SHIFT: usize = 21;
    const THERMAL_MONITOR_AND_SOFTWARE_CONTROLLED_CLOCK_FACILITIES_SHIFT: usize = 22;
    const INTEL_MMX_TECHNOLOGY_SHIFT: usize = 23;
    const FXSAVE_AND_FXRSTORE_INSTRUCTIONS_SHIFT: usize = 24;
    const SSE_SHIFT: usize = 25;
    const SSE2_SHIFT: usize = 26;
    const SELF_SNOOP_SHIFT: usize = 27;
    const MAX_APIC_IDS_RESERVED_FIELD_IS_VALID_SHIFT: usize = 28;
    const THERMAL_MONITOR_SHIFT: usize = 29;
    const PENDING_BREAK_ENABLE_SHIFT: usize = 31;


    const FLOATING_POINT_UNIT_MASK: u32 = (1 << Self::FLOATING_POINT_UNIT_SHIFT) as u32;
    const VIRTUAL_8086_MODE_ENHANCEMENTS_MASK: u32 = (1 << Self::VIRTUAL_8086_MODE_ENHANCEMENTS_SHIFT) as u32;
    const DEBUGGING_EXTENSIONS_MASK: u32 = (1 << Self::DEBUGGING_EXTENSIONS_SHIFT) as u32;
    const PAGE_SIZE_EXTENSION_MASK: u32 = (1 << Self::PAGE_SIZE_EXTENSION_SHIFT) as u32;
    const TIME_STAMP_COUNTER_MASK: u32 = (1 << Self::TIME_STAMP_COUNTER_SHIFT) as u32;
    const MODEL_SPECIFIC_REGISTERS_MASK: u32 = (1 << Self::MODEL_SPECIFIC_REGISTERS_SHIFT) as u32;
    const PHYSICAL_ADDRESS_EXTENSION_MASK: u32 = (1 << Self::PHYSICAL_ADDRESS_EXTENSION_SHIFT) as u32;
    const MACHINE_CHECK_EXCEPTION_MASK: u32 = (1 << Self::MACHINE_CHECK_EXCEPTION_SHIFT) as u32;
    const CMPXCHG8B_INSTRUCTION_MASK: u32 = (1 << Self::CMPXCHG8B_INSTRUCTION_SHIFT) as u32;
    const APIC_MASK: u32 = (1 << Self::APIC_SHIFT) as u32;
    const SYSENTER_AND_SYSEXIT_INSTRUCTIONS_MASK: u32 = (1 << Self::SYSENTER_AND_SYSEXIT_INSTRUCTIONS_SHIFT) as u32;
    const MEMORY_TYPE_RANGE_REGISTERS_MASK: u32 = (1 << Self::MEMORY_TYPE_RANGE_REGISTERS_SHIFT) as u32;
    const PAGE_GLOBAL_BIT_MASK: u32 = (1 << Self::PAGE_GLOBAL_BIT_SHIFT) as u32;
    const MACHINE_CHECK_ARCHITECTURE_MASK: u32 = (1 << Self::MACHINE_CHECK_ARCHITECTURE_SHIFT) as u32;
    const CONDITIONAL_MOVE_INSTRUCTIONS_MASK: u32 = (1 << Self::CONDITIONAL_MOVE_INSTRUCTIONS_SHIFT) as u32;
    const PAGE_ATTRIBUTE_TABLE_MASK: u32 = (1 << Self::PAGE_ATTRIBUTE_TABLE_SHIFT) as u32;
    const PAGE_SIZE_EXTENSION_36BIT_MASK: u32 = (1 << Self::PAGE_SIZE_EXTENSION_36BIT_SHIFT) as u32;
    const PROCESSOR_SERIAL_NUMBER_MASK: u32 = (1 << Self::PROCESSOR_SERIAL_NUMBER_SHIFT) as u32;
    const CLFLUSH_INSTRUCTION_MASK: u32 = (1 << Self::CLFLUSH_INSTRUCTION_SHIFT) as u32;
    const DEBUG_STORE_MASK: u32 = (1 << Self::DEBUG_STORE_SHIFT) as u32;
    const THERMAL_MONITOR_AND_SOFTWARE_CONTROLLED_CLOCK_FACILITIES_MASK: u32 = (1 << Self::THERMAL_MONITOR_AND_SOFTWARE_CONTROLLED_CLOCK_FACILITIES_SHIFT) as u32;
    const INTEL_MMX_TECHNOLOGY_MASK: u32 = (1 << Self::INTEL_MMX_TECHNOLOGY_SHIFT) as u32;
    const FXSAVE_AND_FXRSTORE_INSTRUCTIONS_MASK: u32 = (1 << Self::FXSAVE_AND_FXRSTORE_INSTRUCTIONS_SHIFT) as u32;
    const SSE_MASK: u32 = (1 << Self::SSE_SHIFT) as u32;
    const SSE2_MASK: u32 = (1 << Self::SSE2_SHIFT) as u32;
    const SELF_SNOOP_MASK: u32 = (1 << Self::SELF_SNOOP_SHIFT) as u32;
    const MAX_APIC_IDS_RESERVED_FIELD_IS_VALID_MASK: u32 = (1 << Self::MAX_APIC_IDS_RESERVED_FIELD_IS_VALID_SHIFT) as u32;
    const THERMAL_MONITOR_MASK: u32 = (1 << Self::THERMAL_MONITOR_SHIFT) as u32;
    const PENDING_BREAK_ENABLE_MASK: u32 = (1 << Self::PENDING_BREAK_ENABLE_SHIFT) as u32;
}

impl From<u32> for Edx {
    fn from(edx: u32) -> Self {
        let floating_point_unit: bool = edx & Self::FLOATING_POINT_UNIT_MASK != 0;
        let virtual_8086_mode_enhancements: bool = edx & Self::VIRTUAL_8086_MODE_ENHANCEMENTS_MASK != 0;
        let debugging_extensions: bool = edx & Self::DEBUGGING_EXTENSIONS_MASK != 0;
        let page_size_extension: bool = edx & Self::PAGE_SIZE_EXTENSION_MASK != 0;
        let time_stamp_counter: bool = edx & Self::TIME_STAMP_COUNTER_MASK != 0;
        let model_specific_registers: bool = edx & Self::MODEL_SPECIFIC_REGISTERS_MASK != 0;
        let physical_address_extension: bool = edx & Self::PHYSICAL_ADDRESS_EXTENSION_MASK != 0;
        let machine_check_exception: bool = edx & Self::MACHINE_CHECK_EXCEPTION_MASK != 0;
        let cmpxchg8b_instruction: bool = edx & Self::CMPXCHG8B_INSTRUCTION_MASK != 0;
        let apic: bool = edx & Self::APIC_MASK != 0;
        let sysenter_and_sysexit_instructions: bool = edx & Self::SYSENTER_AND_SYSEXIT_INSTRUCTIONS_MASK != 0;
        let memory_type_range_registers: bool = edx & Self::MEMORY_TYPE_RANGE_REGISTERS_MASK != 0;
        let page_global_bit: bool = edx & Self::PAGE_GLOBAL_BIT_MASK != 0;
        let machine_check_architecture: bool = edx & Self::MACHINE_CHECK_ARCHITECTURE_MASK != 0;
        let conditional_move_instructions: bool = edx & Self::CONDITIONAL_MOVE_INSTRUCTIONS_MASK != 0;
        let page_attribute_table: bool = edx & Self::PAGE_ATTRIBUTE_TABLE_MASK != 0;
        let page_size_extension_36bit: bool = edx & Self::PAGE_SIZE_EXTENSION_36BIT_MASK != 0;
        let processor_serial_number: bool = edx & Self::PROCESSOR_SERIAL_NUMBER_MASK != 0;
        let clflush_instruction: bool = edx & Self::CLFLUSH_INSTRUCTION_MASK != 0;
        let debug_store: bool = edx & Self::DEBUG_STORE_MASK != 0;
        let thermal_monitor_and_software_controlled_clock_facilities: bool = edx & Self::THERMAL_MONITOR_AND_SOFTWARE_CONTROLLED_CLOCK_FACILITIES_MASK != 0;
        let intel_mmx_technology: bool = edx & Self::INTEL_MMX_TECHNOLOGY_MASK != 0;
        let fxsave_and_fxrstore_instructions: bool = edx & Self::FXSAVE_AND_FXRSTORE_INSTRUCTIONS_MASK != 0;
        let sse: bool = edx & Self::SSE_MASK != 0;
        let sse2: bool = edx & Self::SSE2_MASK != 0;
        let self_snoop: bool = edx & Self::SELF_SNOOP_MASK != 0;
        let max_apic_ids_reserved_field_is_valid: bool = edx & Self::MAX_APIC_IDS_RESERVED_FIELD_IS_VALID_MASK != 0;
        let thermal_monitor: bool = edx & Self::THERMAL_MONITOR_MASK != 0;
        let pending_break_enable: bool = edx & Self::PENDING_BREAK_ENABLE_MASK != 0;
        Self {
            floating_point_unit,
            virtual_8086_mode_enhancements,
            debugging_extensions,
            page_size_extension,
            time_stamp_counter,
            model_specific_registers,
            physical_address_extension,
            machine_check_exception,
            cmpxchg8b_instruction,
            apic,
            sysenter_and_sysexit_instructions,
            memory_type_range_registers,
            page_global_bit,
            machine_check_architecture,
            conditional_move_instructions,
            page_attribute_table,
            page_size_extension_36bit,
            processor_serial_number,
            clflush_instruction,
            debug_store,
            thermal_monitor_and_software_controlled_clock_facilities,
            intel_mmx_technology,
            fxsave_and_fxrstore_instructions,
            sse,
            sse2,
            self_snoop,
            max_apic_ids_reserved_field_is_valid,
            thermal_monitor,
            pending_break_enable,
        }
    }
}

