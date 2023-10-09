use super::super::CpuidOutRegisters;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ecx0x00000001 {
    eax: Eax,
    ebx: Ebx,
    edx: Edx,
    ecx: Ecx,
}

impl Ecx0x00000001 {
    pub fn new() -> Self {
        let eax: u32 = 0x0000000d;
        let ecx: u32 = 0x00000001;
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
        Self {
            eax,
            ebx,
            edx,
            ecx,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax {
    xsaveopt: bool,
    xsavec: bool,
    xgetbv: bool,
    xsaves_xrstors_and_ia32_xss: bool,
}

impl Eax {
    const XSAVEOPT_SHIFT: usize = 0;
    const XSAVEC_SHIFT: usize = 1;
    const XGETBV_SHIFT: usize = 2;
    const XSAVES_XRSTORS_AND_IA32_XSS_SHIFT: usize = 3;

    const XSAVEOPT_MASK: u32 = (1 << Self::XSAVEOPT_SHIFT) as u32;
    const XSAVEC_MASK: u32 = (1 << Self::XSAVEC_SHIFT) as u32;
    const XGETBV_MASK: u32 = (1 << Self::XGETBV_SHIFT) as u32;
    const XSAVES_XRSTORS_AND_IA32_XSS_MASK: u32 = (1 << Self::XSAVES_XRSTORS_AND_IA32_XSS_SHIFT) as u32;
}

impl From<u32> for Eax {
    fn from(eax: u32) -> Eax {
        let xsaveopt: bool = eax & Self::XSAVEOPT_MASK != 0;
        let xsavec: bool = eax & Self::XSAVEC_MASK != 0;
        let xgetbv: bool = eax & Self::XGETBV_MASK != 0;
        let xsaves_xrstors_and_ia32_xss: bool = eax & Self::XSAVES_XRSTORS_AND_IA32_XSS_MASK != 0;
        Self {
            xsaveopt,
            xsavec,
            xgetbv,
            xsaves_xrstors_and_ia32_xss,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ebx {
    size_in_bytes_of_the_xsave_area: u32,
}

impl From<u32> for Ebx {
    fn from(ebx: u32) -> Self {
        let size_in_bytes_of_the_xsave_area: u32 = ebx;
        Self {
            size_in_bytes_of_the_xsave_area,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Edx {
    supported_bits_of_the_upper_32_bits_of_the_ia32_xss_msr: u32,
}

impl From<u32> for Edx {
    fn from(edx: u32) -> Self {
        let supported_bits_of_the_upper_32_bits_of_the_ia32_xss_msr: u32 = edx;
        Self {
            supported_bits_of_the_upper_32_bits_of_the_ia32_xss_msr,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ecx {
    used_for_xcr0_0: u8,
    pt_state: bool,
    used_for_xcr0_1: bool,
    cet_user_state: bool,
    cet_supervisor_state: bool,
    hdc_state: bool,
    lbr_state: bool,
    hwp_state: bool,
}

impl Ecx {
    const USED_FOR_XCR0_0_SHIFT: usize = 0;
    const PT_STATE_SHIFT: usize = 8;
    const USED_FOR_XCR0_1_SHIFT: usize = 10;
    const CET_USER_STATE_SHIFT: usize = 11;
    const CET_SUPERVISOR_STATE_SHIFT: usize = 12;
    const HDC_STATE_SHIFT: usize = 13;
    const LBR_STATE_SHIFT: usize = 15;
    const HWP_STATE_SHIFT: usize = 16;

    const USED_FOR_XCR0_0_SHIFT_END: usize = 7;
    const PT_STATE_SHIFT_END: usize = 8;
    const USED_FOR_XCR0_1_SHIFT_END: usize = 10;
    const CET_USER_STATE_SHIFT_END: usize = 11;
    const CET_SUPERVISOR_STATE_SHIFT_END: usize = 12;
    const HDC_STATE_SHIFT_END: usize = 13;
    const LBR_STATE_SHIFT_END: usize = 15;
    const HWP_STATE_SHIFT_END: usize = 16;

    const USED_FOR_XCR0_0_LENGTH: usize = Self::USED_FOR_XCR0_0_SHIFT_END - Self::USED_FOR_XCR0_0_SHIFT + 1;
    const PT_STATE_LENGTH: usize = Self::PT_STATE_SHIFT_END - Self::PT_STATE_SHIFT + 1;
    const USED_FOR_XCR0_1_LENGTH: usize = Self::USED_FOR_XCR0_1_SHIFT_END - Self::USED_FOR_XCR0_1_SHIFT + 1;
    const CET_USER_STATE_LENGTH: usize = Self::CET_USER_STATE_SHIFT_END - Self::CET_USER_STATE_SHIFT + 1;
    const CET_SUPERVISOR_STATE_LENGTH: usize = Self::CET_SUPERVISOR_STATE_SHIFT_END - Self::CET_SUPERVISOR_STATE_SHIFT + 1;
    const HDC_STATE_LENGTH: usize = Self::HDC_STATE_SHIFT_END - Self::HDC_STATE_SHIFT + 1;
    const LBR_STATE_LENGTH: usize = Self::LBR_STATE_SHIFT_END - Self::LBR_STATE_SHIFT + 1;
    const HWP_STATE_LENGTH: usize = Self::HWP_STATE_SHIFT_END - Self::HWP_STATE_SHIFT + 1;

    const USED_FOR_XCR0_0_MASK: u32 = (((1 << Self::USED_FOR_XCR0_0_LENGTH) - 1) << Self::USED_FOR_XCR0_0_SHIFT) as u32;
    const PT_STATE_MASK: u32 = (((1 << Self::PT_STATE_LENGTH) - 1) << Self::PT_STATE_SHIFT) as u32;
    const USED_FOR_XCR0_1_MASK: u32 = (((1 << Self::USED_FOR_XCR0_1_LENGTH) - 1) << Self::USED_FOR_XCR0_1_SHIFT) as u32;
    const CET_USER_STATE_MASK: u32 = (((1 << Self::CET_USER_STATE_LENGTH) - 1) << Self::CET_USER_STATE_SHIFT) as u32;
    const CET_SUPERVISOR_STATE_MASK: u32 = (((1 << Self::CET_SUPERVISOR_STATE_LENGTH) - 1) << Self::CET_SUPERVISOR_STATE_SHIFT) as u32;
    const HDC_STATE_MASK: u32 = (((1 << Self::HDC_STATE_LENGTH) - 1) << Self::HDC_STATE_SHIFT) as u32;
    const LBR_STATE_MASK: u32 = (((1 << Self::LBR_STATE_LENGTH) - 1) << Self::LBR_STATE_SHIFT) as u32;
    const HWP_STATE_MASK: u32 = (((1 << Self::HWP_STATE_LENGTH) - 1) << Self::HWP_STATE_SHIFT) as u32;
}

impl From<u32> for Ecx {
    fn from(ecx: u32) -> Self {
        let used_for_xcr0_0: u8 = ((ecx & Self::USED_FOR_XCR0_0_MASK) >> Self::USED_FOR_XCR0_0_SHIFT) as u8;
        let pt_state: bool = ecx & Self::PT_STATE_MASK != 0;
        let used_for_xcr0_1: bool = ecx & Self::USED_FOR_XCR0_1_MASK != 0;
        let cet_user_state: bool = ecx & Self::CET_USER_STATE_MASK != 0;
        let cet_supervisor_state: bool = ecx & Self::CET_SUPERVISOR_STATE_MASK != 0;
        let hdc_state: bool = ecx & Self::HDC_STATE_MASK != 0;
        let lbr_state: bool = ecx & Self::LBR_STATE_MASK != 0;
        let hwp_state: bool = ecx & Self::HWP_STATE_MASK != 0;
        Self {
            used_for_xcr0_0,
            pt_state,
            used_for_xcr0_1,
            cet_user_state,
            cet_supervisor_state,
            hdc_state,
            lbr_state,
            hwp_state,
        }
    }
}

