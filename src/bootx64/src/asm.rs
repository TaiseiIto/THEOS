use core::arch::asm;

pub type Port = u16;

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
    iopl: bool,
    nt: bool,
    rf: bool,
    vm: bool,
    ac: bool,
    vif: bool,
    vip: bool,
    id: bool,
}

impl Rflags {
    const CF: u64 = 1 << 0;
    const PF: u64 = 1 << 2;
    const AF: u64 = 1 << 4;
    const ZF: u64 = 1 << 6;
    const SF: u64 = 1 << 7;
    const TF: u64 = 1 << 8;
    const IF: u64 = 1 << 9;
    const DF: u64 = 1 << 10;
    const OF: u64 = 1 << 11;
    const IOPL: u64 = (1 << 12) | (1 << 13);
    const NT: u64 = 1 << 14;
    const RF: u64 = 1 << 16;
    const VM: u64 = 1 << 17;
    const AC: u64 = 1 << 18;
    const VIF: u64 = 1 << 19;
    const VIP: u64 = 1 << 20;
    const ID: u64 = 1 << 21;

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
        let iopl: bool = value & Self::IOPL != 0;
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
        let iopl: u64 = match self.iopl {
            true => Rflags::IOPL,
            false => 0,
        };
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

fn get_rflags() -> u64 {
    let mut rflags: u64;
    unsafe {
        asm!(
            "pushfq",
            "pop rax",
            out("rax") rflags,
        );
    }
    rflags
}

fn set_rflags(rflags: u64) {
    unsafe {
        asm!(
            "push rax",
            "popfq",
            in("rax") rflags,
        );
    }
}

pub fn hlt() {
    unsafe {
        asm!("hlt");
    }
}

pub fn inb(port: Port) -> u8 {
    let mut value: u8;
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
        );
    }
    value
}

pub fn outb(port: Port, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
        );
    }
}

