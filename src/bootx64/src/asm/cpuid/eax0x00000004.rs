use super::{
    CpuidOutRegisters,
    eax0x00000000::Eax0x00000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x00000004 {
    eax: Eax,
}

impl Eax0x00000004 {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        if 4 <= eax0x00000000.max_eax() {
            let CpuidOutRegisters {
                eax,
                ebx: _,
                edx: _,
                ecx: _,
            } = CpuidOutRegisters::cpuid(4);
            let eax: Eax = eax.into();
            Some(Self {
                eax,
            })
        } else {
            None
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax {
    cache_type_field: u8,
    cache_level: u8,
    self_initializing_cache_level: bool,
    fully_associative_cache: bool,
    maximum_number_of_addressable_ids_for_logical_processors_sharing_this_cache: u16,
    maximum_number_of_addressable_ids_for_processor_cores_in_the_physical_package: u8,
}

impl Eax {
    const CACHE_TYPE_FIELD_SHIFT: usize = 0;
    const CACHE_LEVEL_SHIFT: usize = 5;
    const SELF_INITIALIZING_CACHE_LEVEL_SHIFT: usize = 8;
    const FULLY_ASSOCIATIVE_CACHE_SHIFT: usize = 9;
    const MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_LOGICAL_PROCESSORS_SHARING_THIS_CACHE_SHIFT: usize = 14;
    const MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_PROCESSOR_CORES_IN_THE_PHYSICAL_PACKAGE_SHIFT: usize = 26;

    const CACHE_TYPE_FIELD_SHIFT_END: usize = 4;
    const CACHE_LEVEL_SHIFT_END: usize = 7;
    const SELF_INITIALIZING_CACHE_LEVEL_SHIFT_END: usize = 8;
    const FULLY_ASSOCIATIVE_CACHE_SHIFT_END: usize = 9;
    const MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_LOGICAL_PROCESSORS_SHARING_THIS_CACHE_SHIFT_END: usize = 25;
    const MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_PROCESSOR_CORES_IN_THE_PHYSICAL_PACKAGE_SHIFT_END: usize = 31;

    const CACHE_TYPE_FIELD_LENGTH: usize = Self::CACHE_TYPE_FIELD_SHIFT_END - Self::CACHE_TYPE_FIELD_SHIFT + 1;
    const CACHE_LEVEL_LENGTH: usize = Self::CACHE_LEVEL_SHIFT_END - Self::CACHE_LEVEL_SHIFT + 1;
    const SELF_INITIALIZING_CACHE_LEVEL_LENGTH: usize = Self::SELF_INITIALIZING_CACHE_LEVEL_SHIFT_END - Self::SELF_INITIALIZING_CACHE_LEVEL_SHIFT + 1;
    const FULLY_ASSOCIATIVE_CACHE_LENGTH: usize = Self::FULLY_ASSOCIATIVE_CACHE_SHIFT_END - Self::FULLY_ASSOCIATIVE_CACHE_SHIFT + 1;
    const MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_LOGICAL_PROCESSORS_SHARING_THIS_CACHE_LENGTH: usize = Self::MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_LOGICAL_PROCESSORS_SHARING_THIS_CACHE_SHIFT_END - Self::MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_LOGICAL_PROCESSORS_SHARING_THIS_CACHE_SHIFT + 1;
    const MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_PROCESSOR_CORES_IN_THE_PHYSICAL_PACKAGE_LENGTH: usize = Self::MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_PROCESSOR_CORES_IN_THE_PHYSICAL_PACKAGE_SHIFT_END - Self::MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_PROCESSOR_CORES_IN_THE_PHYSICAL_PACKAGE_SHIFT + 1;

    const CACHE_TYPE_FIELD_MASK: u32 = (((1 << Self::CACHE_TYPE_FIELD_LENGTH) - 1) << Self::CACHE_TYPE_FIELD_SHIFT) as u32;
    const CACHE_LEVEL_MASK: u32 = (((1 << Self::CACHE_LEVEL_LENGTH) - 1) << Self::CACHE_LEVEL_SHIFT) as u32;
    const SELF_INITIALIZING_CACHE_LEVEL_MASK: u32 = (((1 << Self::SELF_INITIALIZING_CACHE_LEVEL_LENGTH) - 1) << Self::SELF_INITIALIZING_CACHE_LEVEL_SHIFT) as u32;
    const FULLY_ASSOCIATIVE_CACHE_MASK: u32 = (((1 << Self::FULLY_ASSOCIATIVE_CACHE_LENGTH) - 1) << Self::FULLY_ASSOCIATIVE_CACHE_SHIFT) as u32;
    const MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_LOGICAL_PROCESSORS_SHARING_THIS_CACHE_MASK: u32 = (((1 << Self::MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_LOGICAL_PROCESSORS_SHARING_THIS_CACHE_LENGTH) - 1) << Self::MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_LOGICAL_PROCESSORS_SHARING_THIS_CACHE_SHIFT) as u32;
    const MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_PROCESSOR_CORES_IN_THE_PHYSICAL_PACKAGE_MASK: u32 = (((1 << Self::MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_PROCESSOR_CORES_IN_THE_PHYSICAL_PACKAGE_LENGTH) - 1) << Self::MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_PROCESSOR_CORES_IN_THE_PHYSICAL_PACKAGE_SHIFT) as u32;
}

impl From<u32> for Eax {
    fn from(eax: u32) -> Self {
        let cache_type_field = ((eax & Self::CACHE_TYPE_FIELD_MASK) >> Self::CACHE_TYPE_FIELD_SHIFT) as u8;
        let cache_level = ((eax & Self::CACHE_LEVEL_MASK) >> Self::CACHE_LEVEL_SHIFT) as u8;
        let self_initializing_cache_level = eax & Self::SELF_INITIALIZING_CACHE_LEVEL_MASK != 0;
        let fully_associative_cache = eax & Self::FULLY_ASSOCIATIVE_CACHE_MASK != 0;
        let maximum_number_of_addressable_ids_for_logical_processors_sharing_this_cache = ((eax & Self::MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_LOGICAL_PROCESSORS_SHARING_THIS_CACHE_MASK) >> Self::MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_LOGICAL_PROCESSORS_SHARING_THIS_CACHE_SHIFT) as u16;
        let maximum_number_of_addressable_ids_for_processor_cores_in_the_physical_package = ((eax & Self::MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_PROCESSOR_CORES_IN_THE_PHYSICAL_PACKAGE_MASK) >> Self::MAXIMUM_NUMBER_OF_ADDRESSABLE_IDS_FOR_PROCESSOR_CORES_IN_THE_PHYSICAL_PACKAGE_SHIFT) as u8;
        Self {
            cache_type_field,
            cache_level,
            self_initializing_cache_level,
            fully_associative_cache,
            maximum_number_of_addressable_ids_for_logical_processors_sharing_this_cache,
            maximum_number_of_addressable_ids_for_processor_cores_in_the_physical_package,
        }
    }
}

