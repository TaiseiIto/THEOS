// References
// Intel 64 an IA-32 Architectures Software Developer's Manual, Volume 3, Chapter 2.5 Control Registers

use core::arch::asm;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Cr4 {
    vme: bool,
    pvi: bool,
    tsd: bool,
    de: bool,
    pse: bool,
    pae: bool,
    mce: bool,
    pge: bool,
    pce: bool,
    osfxsr: bool,
    osxmmexcpt: bool,
    umip: bool,
    la57: bool,
    vmxe: bool,
    smxe: bool,
    fsgsbase: bool,
    pcide: bool,
    osxsave: bool,
    kl: bool,
    smep: bool,
    smap: bool,
    pke: bool,
    cet: bool,
    pks: bool,
}

impl Cr4 {
    const VME_SHIFT: usize = 0;
    const PVI_SHIFT: usize = 1;
    const TSD_SHIFT: usize = 2;
    const DE_SHIFT: usize = 3;
    const PSE_SHIFT: usize = 4;
    const PAE_SHIFT: usize = 5;
    const MCE_SHIFT: usize = 6;
    const PGE_SHIFT: usize = 7;
    const PCE_SHIFT: usize = 8;
    const OSFXSR_SHIFT: usize = 9;
    const OSXMMEXCPT_SHIFT: usize = 10;
    const UMIP_SHIFT: usize = 11;
    const LA57_SHIFT: usize = 12;
    const VMXE_SHIFT: usize = 13;
    const SMXE_SHIFT: usize = 14;
    const FSGSBASE_SHIFT: usize = 16;
    const PCIDE_SHIFT: usize = 17;
    const OSXSAVE_SHIFT: usize = 18;
    const KL_SHIFT: usize = 19;
    const SMEP_SHIFT: usize = 20;
    const SMAP_SHIFT: usize = 21;
    const PKE_SHIFT: usize = 22;
    const CET_SHIFT: usize = 23;
    const PKS_SHIFT: usize = 24;

    const VME_MASK: u64 = 1 << Self::VME_SHIFT;
    const PVI_MASK: u64 = 1 << Self::PVI_SHIFT;
    const TSD_MASK: u64 = 1 << Self::TSD_SHIFT;
    const DE_MASK: u64 = 1 << Self::DE_SHIFT;
    const PSE_MASK: u64 = 1 << Self::PSE_SHIFT;
    const PAE_MASK: u64 = 1 << Self::PAE_SHIFT;
    const MCE_MASK: u64 = 1 << Self::MCE_SHIFT;
    const PGE_MASK: u64 = 1 << Self::PGE_SHIFT;
    const PCE_MASK: u64 = 1 << Self::PCE_SHIFT;
    const OSFXSR_MASK: u64 = 1 << Self::OSFXSR_SHIFT;
    const OSXMMEXCPT_MASK: u64 = 1 << Self::OSXMMEXCPT_SHIFT;
    const UMIP_MASK: u64 = 1 << Self::UMIP_SHIFT;
    const LA57_MASK: u64 = 1 << Self::LA57_SHIFT;
    const VMXE_MASK: u64 = 1 << Self::VMXE_SHIFT;
    const SMXE_MASK: u64 = 1 << Self::SMXE_SHIFT;
    const FSGSBASE_MASK: u64 = 1 << Self::FSGSBASE_SHIFT;
    const PCIDE_MASK: u64 = 1 << Self::PCIDE_SHIFT;
    const OSXSAVE_MASK: u64 = 1 << Self::OSXSAVE_SHIFT;
    const KL_MASK: u64 = 1 << Self::KL_SHIFT;
    const SMEP_MASK: u64 = 1 << Self::SMEP_SHIFT;
    const SMAP_MASK: u64 = 1 << Self::SMAP_SHIFT;
    const PKE_MASK: u64 = 1 << Self::PKE_SHIFT;
    const CET_MASK: u64 = 1 << Self::CET_SHIFT;
    const PKS_MASK: u64 = 1 << Self::PKS_SHIFT;

    #[allow(dead_code)]
    pub fn get() -> Self {
        let mut cr4: u64;
        unsafe {
            asm!(
                "mov rax, cr4",
                out("rax") cr4,
            );
        }
        let vme: bool = cr4 & Self::VME_MASK != 0;
        let pvi: bool = cr4 & Self::PVI_MASK != 0;
        let tsd: bool = cr4 & Self::TSD_MASK != 0;
        let de: bool = cr4 & Self::DE_MASK != 0;
        let pse: bool = cr4 & Self::PSE_MASK != 0;
        let pae: bool = cr4 & Self::PAE_MASK != 0;
        let mce: bool = cr4 & Self::MCE_MASK != 0;
        let pge: bool = cr4 & Self::PGE_MASK != 0;
        let pce: bool = cr4 & Self::PCE_MASK != 0;
        let osfxsr: bool = cr4 & Self::OSFXSR_MASK != 0;
        let osxmmexcpt: bool = cr4 & Self::OSXMMEXCPT_MASK != 0;
        let umip: bool = cr4 & Self::UMIP_MASK != 0;
        let la57: bool = cr4 & Self::LA57_MASK != 0;
        let vmxe: bool = cr4 & Self::VMXE_MASK != 0;
        let smxe: bool = cr4 & Self::SMXE_MASK != 0;
        let fsgsbase: bool = cr4 & Self::FSGSBASE_MASK != 0;
        let pcide: bool = cr4 & Self::PCIDE_MASK != 0;
        let osxsave: bool = cr4 & Self::OSXSAVE_MASK != 0;
        let kl: bool = cr4 & Self::KL_MASK != 0;
        let smep: bool = cr4 & Self::SMEP_MASK != 0;
        let smap: bool = cr4 & Self::SMAP_MASK != 0;
        let pke: bool = cr4 & Self::PKE_MASK != 0;
        let cet: bool = cr4 & Self::CET_MASK != 0;
        let pks: bool = cr4 & Self::PKS_MASK != 0;
        Self {
            vme,
            pvi,
            tsd,
            de,
            pse,
            pae,
            mce,
            pge,
            pce,
            osfxsr,
            osxmmexcpt,
            umip,
            la57,
            vmxe,
            smxe,
            fsgsbase,
            pcide,
            osxsave,
            kl,
            smep,
            smap,
            pke,
            cet,
            pks,
        }
    }

    #[allow(dead_code)]
    pub fn la57(&self) -> bool {
        self.la57
    }

    #[allow(dead_code)]
    pub fn pae(&self) -> bool {
        self.pae
    }
}

