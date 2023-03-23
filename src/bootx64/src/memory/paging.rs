// References
// Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 3 System Programming Guide, Chapter 4 Paging

use super::super::asm::{
    control::{
        register0::Cr0,
        register4::Cr4,
    },
    msr::architectural::ia32_efer::Ia32Efer,
};

#[derive(Debug)]
pub enum Mode {
    Disable,
    Bit32,
    Pae,
    Level4,
    Level5,
}

impl Mode {
    pub fn get(cr0: &Cr0, cr4: &Cr4, ia32_efer: &Option<Ia32Efer>) -> Self {
        if cr0.pg() {
            if cr4.pae() {
                if ia32_efer
                    .as_ref()
                    .expect("Can't get a paging mode!")
                    .lme() {
                    if cr4.la57() {
                        Self::Level5
                    } else {
                        Self::Level4
                    }
                } else {
                    Self::Pae
                }
            } else {
                Self::Bit32
            }
        } else {
            Self::Disable
        }
    }
}

