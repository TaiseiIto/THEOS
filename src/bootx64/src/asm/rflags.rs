use super::{
    get_rflags,
    set_rflags,
};

// References
// Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1, Chapter 3, Section 4.3.4 RFLAGS Register in 64-Bit Mode
// Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1, Chapter 3, Section 4.3 EFLAGS Register
#[derive(Debug)]
pub struct Rflags {
    cf: bool,
    pf: bool,
    af: bool,
    zf: bool,
    sf: bool,
    tf: bool,
    interrupt_enable: bool,
    df: bool,
    of: bool,
    iopl: u8,
    nt: bool,
    rf: bool,
    vm: bool,
    ac: bool,
    vif: bool,
    vip: bool,
    id: bool,
}

impl Rflags {
    const CF_SHIFT: u8 = 0;
    const PF_SHIFT: u8 = 2;
    const AF_SHIFT: u8 = 4;
    const ZF_SHIFT: u8 = 6;
    const SF_SHIFT: u8 = 7;
    const TF_SHIFT: u8 = 8;
    const IF_SHIFT: u8 = 9;
    const DF_SHIFT: u8 = 10;
    const OF_SHIFT: u8 = 11;
    const IOPL_SHIFT: u8 = 12;
    const NT_SHIFT: u8 = 14;
    const RF_SHIFT: u8 = 16;
    const VM_SHIFT: u8 = 17;
    const AC_SHIFT: u8 = 18;
    const VIF_SHIFT: u8 = 19;
    const VIP_SHIFT: u8 = 20;
    const ID_SHIFT: u8 = 21;

    const CF: u64 = 1 << Self::CF_SHIFT;
    const PF: u64 = 1 << Self::PF_SHIFT;
    const AF: u64 = 1 << Self::AF_SHIFT;
    const ZF: u64 = 1 << Self::ZF_SHIFT;
    const SF: u64 = 1 << Self::SF_SHIFT;
    const TF: u64 = 1 << Self::TF_SHIFT;
    const IF: u64 = 1 << Self::IF_SHIFT;
    const DF: u64 = 1 << Self::DF_SHIFT;
    const OF: u64 = 1 << Self::OF_SHIFT;
    const IOPL: u64 = 3 << Self::IOPL_SHIFT;
    const NT: u64 = 1 << Self::NT_SHIFT;
    const RF: u64 = 1 << Self::RF_SHIFT;
    const VM: u64 = 1 << Self::VM_SHIFT;
    const AC: u64 = 1 << Self::AC_SHIFT;
    const VIF: u64 = 1 << Self::VIF_SHIFT;
    const VIP: u64 = 1 << Self::VIP_SHIFT;
    const ID: u64 = 1 << Self::ID_SHIFT;

    pub fn get() -> Self {
        get_rflags().into()
    }

    pub fn set(&self) {
        set_rflags(self.into());
    }

    pub fn cpuid_is_supported() -> bool {
        let mut rflags = Self::get();
        rflags.id = true;
        Self::set(&rflags);
        let mut rflags = Self::get();
        match rflags.id {
            true => {
                rflags.id = false;
                Self::set(&rflags);
                let rflags = Self::get();
                match rflags.id {
                    true => false,
                    false => true,
                }
            },
            false => false,
        }
    }
}

impl From<u64> for Rflags {
    fn from(value: u64) -> Self {
        let cf: bool = value & Self::CF != 0;
        let pf: bool = value & Self::PF != 0;
        let af: bool = value & Self::AF != 0;
        let zf: bool = value & Self::ZF != 0;
        let sf: bool = value & Self::SF != 0;
        let tf: bool = value & Self::TF != 0;
        let interrupt_enable: bool = value & Self::IF != 0;
        let df: bool = value & Self::DF != 0;
        let of: bool = value & Self::OF != 0;
        let iopl: u8 = ((value & Self::IOPL) >> Self::IOPL_SHIFT) as u8;
        let nt: bool = value & Self::NT != 0;
        let rf: bool = value & Self::RF != 0;
        let vm: bool = value & Self::VM != 0;
        let ac: bool = value & Self::AC != 0;
        let vif: bool = value & Self::VIF != 0;
        let vip: bool = value & Self::VIP != 0;
        let id: bool = value & Self::ID != 0;
        Self {
            cf,
            pf,
            af,
            zf,
            sf,
            tf,
            interrupt_enable,
            df,
            of,
            iopl,
            nt,
            rf,
            vm,
            ac,
            vif,
            vip,
            id,
        }
    }
}

impl Into<u64> for &Rflags {
    fn into(self) -> u64 {
        let cf: u64 = match self.cf {
            true => Rflags::CF,
            false => 0,
        };
        let pf: u64 = match self.pf {
            true => Rflags::PF,
            false => 0,
        };
        let af: u64 = match self.af {
            true => Rflags::AF,
            false => 0,
        };
        let zf: u64 = match self.zf {
            true => Rflags::ZF,
            false => 0,
        };
        let sf: u64 = match self.sf {
            true => Rflags::SF,
            false => 0,
        };
        let tf: u64 = match self.tf {
            true => Rflags::TF,
            false => 0,
        };
        let interrupt_enable: u64 = match self.interrupt_enable {
            true => Rflags::IF,
            false => 0,
        };
        let df: u64 = match self.df {
            true => Rflags::DF,
            false => 0,
        };
        let of: u64 = match self.of {
            true => Rflags::OF,
            false => 0,
        };
        let iopl: u64 = (self.iopl as u64) << Rflags::IOPL_SHIFT;
        let nt: u64 = match self.nt {
            true => Rflags::NT,
            false => 0,
        };
        let rf: u64 = match self.rf {
            true => Rflags::RF,
            false => 0,
        };
        let vm: u64 = match self.vm {
            true => Rflags::VM,
            false => 0,
        };
        let ac: u64 = match self.ac {
            true => Rflags::AC,
            false => 0,
        };
        let vif: u64 = match self.vif {
            true => Rflags::VIF,
            false => 0,
        };
        let vip: u64 = match self.vip {
            true => Rflags::VIP,
            false => 0,
        };
        let id: u64 = match self.id {
            true => Rflags::ID,
            false => 0,
        };
        cf + pf + af + zf + sf + tf + interrupt_enable + df + of + iopl + nt + rf + vm + ac + vif + vip + id
    }
}


