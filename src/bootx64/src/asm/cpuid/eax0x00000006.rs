use super::{
    CpuidOutRegisters,
    eax0x00000000::Eax0x00000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x00000006 {
    eax: Eax,
    ebx: Ebx,
    edx: Edx,
    ecx: Ecx,
}

impl Eax0x00000006 {
    pub fn new(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 6;
        let ecx: u32 = 0;
        if eax <= eax0x00000000.max_eax() {
            let CpuidOutRegisters {
                eax,
                ebx,
                edx,
                ecx,
            } = CpuidOutRegisters::cpuid(eax, ecx);
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
    digital_temperature_sensor: bool,
    intel_turbo_boost_technology: bool,
    arat: bool,
    pln: bool,
    ecmd: bool,
    ptm: bool,
    hwp: bool,
    hwp_notification: bool,
    hwp_activity_window: bool,
    hwp_energy_performance_preference: bool,
    hwp_package_level_request: bool,
    hdc: bool,
    intel_turbo_boost_max_technology_3: bool,
    hwp_capabilities: bool,
    hwp_peci_override: bool,
    flexible_hwp: bool,
    fast_access_mode: bool,
    hw_feedback: bool,
    ignoring_idle_logical_processor_hwp_request: bool,
    intel_thread_director: bool,
}

impl Eax {
    const DIGITAL_TEMPERATURE_SENSOR_SHIFT: usize = 0;
    const INTEL_TURBO_BOOST_TECHNOLOGY_SHIFT: usize = 1;
    const ARAT_SHIFT: usize = 2;
    const PLN_SHIFT: usize = 4;
    const ECMD_SHIFT: usize = 5;
    const PTM_SHIFT: usize = 6;
    const HWP_SHIFT: usize = 7;
    const HWP_NOTIFICATION_SHIFT: usize = 8;
    const HWP_ACTIVITY_WINDOW_SHIFT: usize = 9;
    const HWP_ENERGY_PERFORMANCE_PREFERENCE_SHIFT: usize = 10;
    const HWP_PACKAGE_LEVEL_REQUEST_SHIFT: usize = 11;
    const HDC_SHIFT: usize = 13;
    const INTEL_TURBO_BOOST_MAX_TECHNOLOGY_3_SHIFT: usize = 14;
    const HWP_CAPABILITIES_SHIFT: usize = 15;
    const HWP_PECI_OVERRIDE_SHIFT: usize = 16;
    const FLEXIBLE_HWP_SHIFT: usize = 17;
    const FAST_ACCESS_MODE_SHIFT: usize = 18;
    const HW_FEEDBACK_SHIFT: usize = 19;
    const IGNORING_IDLE_LOGICAL_PROCESSOR_HWP_REQUEST_SHIFT: usize = 20;
    const INTEL_THREAD_DIRECTOR_SHIFT: usize = 23;

    const DIGITAL_TEMPERATURE_SENSOR_MASK: u32 = (1 << Self::DIGITAL_TEMPERATURE_SENSOR_SHIFT) as u32;
    const INTEL_TURBO_BOOST_TECHNOLOGY_MASK: u32 = (1 << Self::INTEL_TURBO_BOOST_TECHNOLOGY_SHIFT) as u32;
    const ARAT_MASK: u32 = (1 << Self::ARAT_SHIFT) as u32;
    const PLN_MASK: u32 = (1 << Self::PLN_SHIFT) as u32;
    const ECMD_MASK: u32 = (1 << Self::ECMD_SHIFT) as u32;
    const PTM_MASK: u32 = (1 << Self::PTM_SHIFT) as u32;
    const HWP_MASK: u32 = (1 << Self::HWP_SHIFT) as u32;
    const HWP_NOTIFICATION_MASK: u32 = (1 << Self::HWP_NOTIFICATION_SHIFT) as u32;
    const HWP_ACTIVITY_WINDOW_MASK: u32 = (1 << Self::HWP_ACTIVITY_WINDOW_SHIFT) as u32;
    const HWP_ENERGY_PERFORMANCE_PREFERENCE_MASK: u32 = (1 << Self::HWP_ENERGY_PERFORMANCE_PREFERENCE_SHIFT) as u32;
    const HWP_PACKAGE_LEVEL_REQUEST_MASK: u32 = (1 << Self::HWP_PACKAGE_LEVEL_REQUEST_SHIFT) as u32;
    const HDC_MASK: u32 = (1 << Self::HDC_SHIFT) as u32;
    const INTEL_TURBO_BOOST_MAX_TECHNOLOGY_3_MASK: u32 = (1 << Self::INTEL_TURBO_BOOST_MAX_TECHNOLOGY_3_SHIFT) as u32;
    const HWP_CAPABILITIES_MASK: u32 = (1 << Self::HWP_CAPABILITIES_SHIFT) as u32;
    const HWP_PECI_OVERRIDE_MASK: u32 = (1 << Self::HWP_PECI_OVERRIDE_SHIFT) as u32;
    const FLEXIBLE_HWP_MASK: u32 = (1 << Self::FLEXIBLE_HWP_SHIFT) as u32;
    const FAST_ACCESS_MODE_MASK: u32 = (1 << Self::FAST_ACCESS_MODE_SHIFT) as u32;
    const HW_FEEDBACK_MASK: u32 = (1 << Self::HW_FEEDBACK_SHIFT) as u32;
    const IGNORING_IDLE_LOGICAL_PROCESSOR_HWP_REQUEST_MASK: u32 = (1 << Self::IGNORING_IDLE_LOGICAL_PROCESSOR_HWP_REQUEST_SHIFT) as u32;
    const INTEL_THREAD_DIRECTOR_MASK: u32 = (1 << Self::INTEL_THREAD_DIRECTOR_SHIFT) as u32;
}

impl From<u32> for Eax {
    fn from(eax: u32) -> Self {
        let digital_temperature_sensor = eax & Self::DIGITAL_TEMPERATURE_SENSOR_MASK != 0;
        let intel_turbo_boost_technology = eax & Self::INTEL_TURBO_BOOST_TECHNOLOGY_MASK != 0;
        let arat = eax & Self::ARAT_MASK != 0;
        let pln = eax & Self::PLN_MASK != 0;
        let ecmd = eax & Self::ECMD_MASK != 0;
        let ptm = eax & Self::PTM_MASK != 0;
        let hwp = eax & Self::HWP_MASK != 0;
        let hwp_notification = eax & Self::HWP_NOTIFICATION_MASK != 0;
        let hwp_activity_window = eax & Self::HWP_ACTIVITY_WINDOW_MASK != 0;
        let hwp_energy_performance_preference = eax & Self::HWP_ENERGY_PERFORMANCE_PREFERENCE_MASK != 0;
        let hwp_package_level_request = eax & Self::HWP_PACKAGE_LEVEL_REQUEST_MASK != 0;
        let hdc = eax & Self::HDC_MASK != 0;
        let intel_turbo_boost_max_technology_3 = eax & Self::INTEL_TURBO_BOOST_MAX_TECHNOLOGY_3_MASK != 0;
        let hwp_capabilities = eax & Self::HWP_CAPABILITIES_MASK != 0;
        let hwp_peci_override = eax & Self::HWP_PECI_OVERRIDE_MASK != 0;
        let flexible_hwp = eax & Self::FLEXIBLE_HWP_MASK != 0;
        let fast_access_mode = eax & Self::FAST_ACCESS_MODE_MASK != 0;
        let hw_feedback = eax & Self::HW_FEEDBACK_MASK != 0;
        let ignoring_idle_logical_processor_hwp_request = eax & Self::IGNORING_IDLE_LOGICAL_PROCESSOR_HWP_REQUEST_MASK != 0;
        let intel_thread_director = eax & Self::INTEL_THREAD_DIRECTOR_MASK != 0;
        Self {
            digital_temperature_sensor,
            intel_turbo_boost_technology,
            arat,
            pln,
            ecmd,
            ptm,
            hwp,
            hwp_notification,
            hwp_activity_window,
            hwp_energy_performance_preference,
            hwp_package_level_request,
            hdc,
            intel_turbo_boost_max_technology_3,
            hwp_capabilities,
            hwp_peci_override,
            flexible_hwp,
            fast_access_mode,
            hw_feedback,
            ignoring_idle_logical_processor_hwp_request,
            intel_thread_director,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ebx {
    interpret_thresholds: u8,
}

impl From<u32> for Ebx {
    fn from(eax: u32) -> Self {
        let interpret_thresholds: u8 = (eax & 0xf) as u8;
        Self {
            interpret_thresholds,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Edx {
    bitmap: u8,
    enumerates: u8,
    index: u16,
}

impl Edx {
    const BITMAP_SHIFT: usize = 0;
    const ENUMERATES_SHIFT: usize = 8;
    const INDEX_SHIFT: usize = 16;

    const BITMAP_SHIFT_END: usize = 7;
    const ENUMERATES_SHIFT_END: usize = 11;
    const INDEX_SHIFT_END: usize = 31;

    const BITMAP_LENGTH: usize = Self::BITMAP_SHIFT_END - Self::BITMAP_SHIFT + 1;
    const ENUMERATES_LENGTH: usize = Self::ENUMERATES_SHIFT_END - Self::ENUMERATES_SHIFT + 11;
    const INDEX_LENGTH: usize = Self::INDEX_SHIFT_END - Self::INDEX_SHIFT + 11;

    const BITMAP_MASK: u32 = (((1 << Self::BITMAP_LENGTH) - 1) << Self::BITMAP_SHIFT) as u32;
    const ENUMERATES_MASK: u32 = (((1 << Self::ENUMERATES_LENGTH) - 1) << Self::ENUMERATES_SHIFT) as u32;
    const INDEX_MASK: u32 = (((1 << Self::INDEX_LENGTH) - 1) << Self::INDEX_SHIFT) as u32;
}

impl From<u32> for Edx {
    fn from(edx: u32) -> Self {
        let bitmap = ((edx & Self::BITMAP_MASK) >> Self::BITMAP_SHIFT) as u8;
        let enumerates = ((edx & Self::ENUMERATES_MASK) >> Self::ENUMERATES_SHIFT) as u8;
        let index = ((edx & Self::INDEX_MASK) >> Self::INDEX_SHIFT) as u16;
        Self {
            bitmap,
            enumerates,
            index,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ecx {
    hardware_coordination_feedback_capability: bool,
    performance_energy_bias_preference: bool,
    intel_thread_director_classes: u8,
}

impl Ecx {
    const HARDWARE_COORDINATION_FEEDBACK_CAPABILITY_SHIFT: usize = 0;
    const PERFORMANCE_ENERGY_BIAS_PREFERENCE_SHIFT: usize = 3;
    const INTEL_THREAD_DIRECTOR_CLASSES_SHIFT: usize = 8;

    const HARDWARE_COORDINATION_FEEDBACK_CAPABILITY_SHIFT_END: usize = 0;
    const PERFORMANCE_ENERGY_BIAS_PREFERENCE_SHIFT_END: usize = 3;
    const INTEL_THREAD_DIRECTOR_CLASSES_SHIFT_END: usize = 15;

    const HARDWARE_COORDINATION_FEEDBACK_CAPABILITY_LENGTH: usize = Self::HARDWARE_COORDINATION_FEEDBACK_CAPABILITY_SHIFT_END - Self::HARDWARE_COORDINATION_FEEDBACK_CAPABILITY_SHIFT + 1;
    const PERFORMANCE_ENERGY_BIAS_PREFERENCE_LENGTH: usize = Self::PERFORMANCE_ENERGY_BIAS_PREFERENCE_SHIFT_END - Self::PERFORMANCE_ENERGY_BIAS_PREFERENCE_SHIFT + 1;
    const INTEL_THREAD_DIRECTOR_CLASSES_LENGTH: usize = Self::INTEL_THREAD_DIRECTOR_CLASSES_SHIFT_END - Self::INTEL_THREAD_DIRECTOR_CLASSES_SHIFT + 1;

    const HARDWARE_COORDINATION_FEEDBACK_CAPABILITY_MASK: u32 = (((1 << Self::HARDWARE_COORDINATION_FEEDBACK_CAPABILITY_LENGTH) - 1) << Self::HARDWARE_COORDINATION_FEEDBACK_CAPABILITY_SHIFT) as u32;
    const PERFORMANCE_ENERGY_BIAS_PREFERENCE_MASK: u32 = (((1 << Self::PERFORMANCE_ENERGY_BIAS_PREFERENCE_LENGTH) - 1) << Self::PERFORMANCE_ENERGY_BIAS_PREFERENCE_SHIFT) as u32;
    const INTEL_THREAD_DIRECTOR_CLASSES_MASK: u32 = (((1 << Self::INTEL_THREAD_DIRECTOR_CLASSES_LENGTH) - 1) << Self::INTEL_THREAD_DIRECTOR_CLASSES_SHIFT) as u32;
}

impl From<u32> for Ecx {
    fn from(ecx: u32) -> Self {
        let hardware_coordination_feedback_capability = ecx & Self::HARDWARE_COORDINATION_FEEDBACK_CAPABILITY_MASK != 0;
        let performance_energy_bias_preference = ecx & Self::PERFORMANCE_ENERGY_BIAS_PREFERENCE_MASK != 0;
        let intel_thread_director_classes = ((ecx & Self::INTEL_THREAD_DIRECTOR_CLASSES_MASK) >> Self::INTEL_THREAD_DIRECTOR_CLASSES_SHIFT) as u8;
        Self {
            hardware_coordination_feedback_capability,
            performance_energy_bias_preference,
            intel_thread_director_classes,
        }
    }
}

