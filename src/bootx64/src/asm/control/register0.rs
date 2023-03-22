// References
// Intel 64 an IA-32 Architectures Software Developer's Manual, Volume 3, Chapter 2.5 Control Registers

use core::arch::asm;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Cr0 {
    pe: bool,
    mp: bool,
    em: bool,
    ts: bool,
    et: bool,
    ne: bool,
    wp: bool,
    am: bool,
    nw: bool,
    cd: bool,
    pg: bool,
}

impl Cr0 {
    const PE_SHIFT: usize = 0;
    const MP_SHIFT: usize = 1;
    const EM_SHIFT: usize = 2;
    const TS_SHIFT: usize = 3;
    const ET_SHIFT: usize = 4;
    const NE_SHIFT: usize = 5;
    const WP_SHIFT: usize = 16;
    const AM_SHIFT: usize = 18;
    const NW_SHIFT: usize = 29;
    const CD_SHIFT: usize = 30;
    const PG_SHIFT: usize = 31;
    
    const PE_MASK: u64 = 1 << Self::PE_SHIFT;
    const MP_MASK: u64 = 1 << Self::MP_SHIFT;
    const EM_MASK: u64 = 1 << Self::EM_SHIFT;
    const TS_MASK: u64 = 1 << Self::TS_SHIFT;
    const ET_MASK: u64 = 1 << Self::ET_SHIFT;
    const NE_MASK: u64 = 1 << Self::NE_SHIFT;
    const WP_MASK: u64 = 1 << Self::WP_SHIFT;
    const AM_MASK: u64 = 1 << Self::AM_SHIFT;
    const NW_MASK: u64 = 1 << Self::NW_SHIFT;
    const CD_MASK: u64 = 1 << Self::CD_SHIFT;
    const PG_MASK: u64 = 1 << Self::PG_SHIFT;

    pub fn get() -> Self {
        let mut cr0: u64;
        unsafe {
            asm!(
                "mov rax, cr0",
                out("rax") cr0,
            );
        }
        let pe: bool = cr0 & Self::PE_MASK != 0;
        let mp: bool = cr0 & Self::MP_MASK != 0;
        let em: bool = cr0 & Self::EM_MASK != 0;
        let ts: bool = cr0 & Self::TS_MASK != 0;
        let et: bool = cr0 & Self::ET_MASK != 0;
        let ne: bool = cr0 & Self::NE_MASK != 0;
        let wp: bool = cr0 & Self::WP_MASK != 0;
        let am: bool = cr0 & Self::AM_MASK != 0;
        let nw: bool = cr0 & Self::NW_MASK != 0;
        let cd: bool = cr0 & Self::CD_MASK != 0;
        let pg: bool = cr0 & Self::PG_MASK != 0;
        Self {
            pe,
            mp,
            em,
            ts,
            et,
            ne,
            wp,
            am,
            nw,
            cd,
            pg,
        }
    }
}

