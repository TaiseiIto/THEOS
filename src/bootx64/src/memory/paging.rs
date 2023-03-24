// References
// Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 3 System Programming Guide, Chapter 4 Paging

pub mod level4;

use super::super::asm::{
    control::{
        register0::Cr0,
        register3::Cr3,
        register4::Cr4,
    },
    msr::architectural::ia32_efer::Ia32Efer,
};

#[derive(Debug)]
pub enum Paging {
    Disable,
    Bit32,
    Pae,
    Level4 {
        cr3: level4::Cr3,
    },
    Level5,
}

impl Paging {
    pub fn get(cr0: &Cr0, cr3: &Cr3, cr4: &Cr4, ia32_efer: &Option<Ia32Efer>) -> Self {
        if cr0.pg() {
            if cr4.pae() {
                if ia32_efer
                    .as_ref()
                    .expect("Can't get a paging mode!")
                    .lme() {
                    if cr4.la57() {
                        Self::Level5
                    } else {
                        let cr3: u64 = cr3.into();
                        let cr3: level4::Cr3 = cr3.into();
                        Self::Level4 {
                            cr3,
                        }
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
