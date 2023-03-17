use super::{
    CpuidOutRegisters,
    eax0x80000000::Eax0x80000000,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Eax0x80000001 {
    eax: Eax,
    edx: Edx,
    ecx: Ecx,
}

impl Eax0x80000001 {
    pub fn new(eax0x80000000: &Eax0x80000000) -> Option<Self> {
        let eax: u32 = 0x80000001;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x80000000.max_eax() {
            let CpuidOutRegisters {
                eax,
                ebx: _,
                edx,
                ecx,
            } = CpuidOutRegisters::cpuid(eax, ecx);
            let eax: Eax = eax.into();
            let edx: Edx = edx.into();
            let ecx: Ecx = ecx.into();
            Some(Self {
                eax,
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
    extended_processor_signature: u32,
}

impl From<u32> for Eax {
    fn from(eax: u32) -> Self {
        let extended_processor_signature: u32 = eax;
        Self {
            extended_processor_signature,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Edx {
    syscall_sysret: bool,
    execute_disable_bit: bool,
    gbyte_pages: bool,
    rdtscp_and_ia32_tsc_aux: bool,
    intel_64_architecture: bool,
}

impl Edx {
    const SYSCALL_SYSRET_SHIFT: usize = 11;
    const EXECUTE_DISABLE_BIT_SHIFT: usize = 20;
    const GBYTE_PAGES_SHIFT: usize = 26;
    const RDTSCP_AND_IA32_TSC_AUX_SHIFT: usize = 27;
    const INTEL_64_ARCHITECTURE_SHIFT: usize = 29;

    const SYSCALL_SYSRET_MASK: u32 = (1 << Self::SYSCALL_SYSRET_SHIFT) as u32;
    const EXECUTE_DISABLE_BIT_MASK: u32 = (1 << Self::EXECUTE_DISABLE_BIT_SHIFT) as u32;
    const GBYTE_PAGES_MASK: u32 = (1 << Self::GBYTE_PAGES_SHIFT) as u32;
    const RDTSCP_AND_IA32_TSC_AUX_MASK: u32 = (1 << Self::RDTSCP_AND_IA32_TSC_AUX_SHIFT) as u32;
    const INTEL_64_ARCHITECTURE_MASK: u32 = (1 << Self::INTEL_64_ARCHITECTURE_SHIFT) as u32;
}

impl From<u32> for Edx {
    fn from(edx: u32) -> Self {
        let syscall_sysret: bool = edx & Self::SYSCALL_SYSRET_MASK != 0;
        let execute_disable_bit: bool = edx & Self::EXECUTE_DISABLE_BIT_MASK != 0;
        let gbyte_pages: bool = edx & Self::GBYTE_PAGES_MASK != 0;
        let rdtscp_and_ia32_tsc_aux: bool = edx & Self::RDTSCP_AND_IA32_TSC_AUX_MASK != 0;
        let intel_64_architecture: bool = edx & Self::INTEL_64_ARCHITECTURE_MASK != 0;
        Self {
            syscall_sysret,
            execute_disable_bit,
            gbyte_pages,
            rdtscp_and_ia32_tsc_aux,
            intel_64_architecture,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Ecx {
    lahf_sahf: bool,
    lzcnt: bool,
    prefetchw: bool,
}

impl Ecx {
    const LAHF_SAHF_SHIFT: usize = 0;
    const LZCNT_SHIFT: usize = 5;
    const PREFETCHW_SHIFT: usize = 8;

    const LAHF_SAHF_MASK: u32 = (1 << Self::LAHF_SAHF_SHIFT) as u32;
    const LZCNT_MASK: u32 = (1 << Self::LZCNT_SHIFT) as u32;
    const PREFETCHW_MASK: u32 = (1 << Self::PREFETCHW_SHIFT) as u32;
}

impl From<u32> for Ecx {
    fn from(ecx: u32) -> Self {
        let lahf_sahf: bool = ecx & Self::LAHF_SAHF_MASK != 0;
        let lzcnt: bool = ecx & Self::LZCNT_MASK != 0;
        let prefetchw: bool = ecx & Self::PREFETCHW_MASK != 0;
        Self {
            lahf_sahf,
            lzcnt,
            prefetchw,
        }
    }
}

