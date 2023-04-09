// References
// Intel 64 an IA-32 Architectures Software Developer's Manual 3.3.4.5 Segment Discriptor

use {
    alloc::vec::Vec,
    core::{
        arch::asm,
        mem,
        slice,
    },
};

#[derive(Debug)]
pub struct Register {
    base: u64,
    limit: u16,
}

impl Register {
    pub fn get() -> Self {
        let mut gdtr: u128 = 0;
        let gdtrp: &mut u128 = &mut gdtr;
        let gdtrp: *mut u128 = gdtrp as *mut u128;
        let gdtrp: usize = gdtrp as usize;
        unsafe {
            asm!(
                "sgdt [{}]",
                in(reg) gdtrp,
            );
        }
        let base: u64 = (gdtr >> 16) as u64;
        let limit: u16 = gdtr as u16;
        Self {
            base,
            limit,
        }
    }
}

impl Into<&'static [u64]> for Register {
    fn into(self) -> &'static [u64] {
        let base: *const u64 = self.base as *const u64;
        let limit: usize = self.limit as usize;
        let length: usize = (limit + 1) / mem::size_of::<u64>();
        unsafe {
            slice::from_raw_parts(base, length)
        }
    }
}

impl Into<Vec<Descriptor>> for Register {
    fn into(self) -> Vec<Descriptor> {
        let descriptors: &[u64] = self.into();
        descriptors
            .iter()
            .map(|descriptor| (*descriptor).into())
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Descriptor {
    base: u32,
    limit: u32,
    segment_type: u8,
    s: bool,
    dpl: u8,
    p: bool,
    avl: bool,
    l: bool,
    db: bool,
    g: bool,
}

impl Descriptor {
    const LIMIT_LOW_SHIFT_BEGIN: usize = 0;
    const BASE_LOW_SHIFT_BEGIN: usize = 16;
    const SEGMENT_TYPE_SHIFT_BEGIN: usize = 4 * 8 + 8;
    const S_SHIFT_BEGIN: usize = 4 * 8 + 12;
    const DPL_SHIFT_BEGIN: usize = 4 * 8 + 13;
    const P_SHIFT_BEGIN: usize = 4 * 8 + 15;
    const LIMIT_HIGH_SHIFT_BEGIN: usize = 4 * 8 + 16;
    const AVL_SHIFT_BEGIN: usize = 4 * 8 + 20;
    const L_SHIFT_BEGIN: usize = 4 * 8 + 21;
    const DB_SHIFT_BEGIN: usize = 4 * 8 + 22;
    const G_SHIFT_BEGIN: usize = 4 * 8 + 23;
    const BASE_HIGH_SHIFT_BEGIN: usize = 4 * 8 + 24;

    const LIMIT_LOW_SHIFT_END: usize = 16;
    const BASE_LOW_SHIFT_END: usize = 4 * 8 + 8;
    const SEGMENT_TYPE_SHIFT_END: usize = 4 * 8 + 12;
    const S_SHIFT_END: usize = 4 * 8 + 13;
    const DPL_SHIFT_END: usize = 4 * 8 + 15;
    const P_SHIFT_END: usize = 4 * 8 + 16;
    const LIMIT_HIGH_SHIFT_END: usize = 4 * 8 + 20;
    const AVL_SHIFT_END: usize = 4 * 8 + 21;
    const L_SHIFT_END: usize = 4 * 8 + 22;
    const DB_SHIFT_END: usize = 4 * 8 + 23;
    const G_SHIFT_END: usize = 4 * 8 + 24;
    const BASE_HIGH_SHIFT_END: usize = 64;

    const LIMIT_LOW_LENGTH: usize = Self::LIMIT_LOW_SHIFT_END - Self::LIMIT_LOW_SHIFT_BEGIN;
    const BASE_LOW_LENGTH: usize = Self::BASE_LOW_SHIFT_END - Self::BASE_LOW_SHIFT_BEGIN;
    const SEGMENT_TYPE_LENGTH: usize = Self::SEGMENT_TYPE_SHIFT_END - Self::SEGMENT_TYPE_SHIFT_BEGIN;
    const S_LENGTH: usize = Self::S_SHIFT_END - Self::S_SHIFT_BEGIN;
    const DPL_LENGTH: usize = Self::DPL_SHIFT_END - Self::DPL_SHIFT_BEGIN;
    const P_LENGTH: usize = Self::P_SHIFT_END - Self::P_SHIFT_BEGIN;
    const LIMIT_HIGH_LENGTH: usize = Self::LIMIT_HIGH_SHIFT_END - Self::LIMIT_HIGH_SHIFT_BEGIN;
    const AVL_LENGTH: usize = Self::AVL_SHIFT_END - Self::AVL_SHIFT_BEGIN;
    const L_LENGTH: usize = Self::L_SHIFT_END - Self::L_SHIFT_BEGIN;
    const DB_LENGTH: usize = Self::DB_SHIFT_END - Self::DB_SHIFT_BEGIN;
    const G_LENGTH: usize = Self::G_SHIFT_END - Self::G_SHIFT_BEGIN;
    const BASE_HIGH_LENGTH: usize = Self::BASE_HIGH_SHIFT_END - Self::BASE_HIGH_SHIFT_BEGIN;

    const LIMIT_LOW_MASK: u64 = ((1u64 << Self::LIMIT_LOW_LENGTH) - 1) << Self::LIMIT_LOW_SHIFT_BEGIN;
    const BASE_LOW_MASK: u64 = ((1u64 << Self::BASE_LOW_LENGTH) - 1) << Self::BASE_LOW_SHIFT_BEGIN;
    const SEGMENT_TYPE_MASK: u64 = ((1u64 << Self::SEGMENT_TYPE_LENGTH) - 1) << Self::SEGMENT_TYPE_SHIFT_BEGIN;
    const S_MASK: u64 = ((1u64 << Self::S_LENGTH) - 1) << Self::S_SHIFT_BEGIN;
    const DPL_MASK: u64 = ((1u64 << Self::DPL_LENGTH) - 1) << Self::DPL_SHIFT_BEGIN;
    const P_MASK: u64 = ((1u64 << Self::P_LENGTH) - 1) << Self::P_SHIFT_BEGIN;
    const LIMIT_HIGH_MASK: u64 = ((1u64 << Self::LIMIT_HIGH_LENGTH) - 1) << Self::LIMIT_HIGH_SHIFT_BEGIN;
    const AVL_MASK: u64 = ((1u64 << Self::AVL_LENGTH) - 1) << Self::AVL_SHIFT_BEGIN;
    const L_MASK: u64 = ((1u64 << Self::L_LENGTH) - 1) << Self::L_SHIFT_BEGIN;
    const DB_MASK: u64 = ((1u64 << Self::DB_LENGTH) - 1) << Self::DB_SHIFT_BEGIN;
    const G_MASK: u64 = ((1u64 << Self::G_LENGTH) - 1) << Self::G_SHIFT_BEGIN;
    const BASE_HIGH_MASK: u64 = ((1u64 << Self::BASE_HIGH_LENGTH) - 1) << Self::BASE_HIGH_SHIFT_BEGIN;
}

impl From<u64> for Descriptor {
    fn from(descriptor: u64) -> Self {
        let base_low: u32 = ((descriptor & Self::BASE_LOW_MASK) >> Self::BASE_LOW_SHIFT_BEGIN) as u32;
        let base_high: u32 = ((descriptor & Self::BASE_HIGH_MASK) >> Self::BASE_HIGH_SHIFT_BEGIN) as u32;
        let base: u32 = (base_high << Self::BASE_LOW_LENGTH) | base_low;
        let limit_low: u32 = ((descriptor & Self::LIMIT_LOW_MASK) >> Self::LIMIT_LOW_SHIFT_BEGIN) as u32;
        let limit_high: u32 = ((descriptor & Self::LIMIT_HIGH_MASK) >> Self::LIMIT_HIGH_SHIFT_BEGIN) as u32;
        let limit: u32 = (limit_high << Self::LIMIT_LOW_LENGTH) | limit_low;
        let segment_type: u8 = ((descriptor & Self::SEGMENT_TYPE_MASK) >> Self::SEGMENT_TYPE_SHIFT_BEGIN) as u8;
        let s: bool = descriptor & Self::S_MASK != 0;
        let dpl: u8 = ((descriptor & Self::DPL_MASK) >> Self::DPL_SHIFT_BEGIN) as u8;
        let p: bool = descriptor & Self::P_MASK != 0;
        let avl: bool = descriptor & Self::AVL_MASK != 0;
        let l: bool = descriptor & Self::L_MASK != 0;
        let db: bool = descriptor & Self::DB_MASK != 0;
        let g: bool = descriptor & Self::G_MASK != 0;
        Self {
            base,
            limit,
            segment_type,
            s,
            dpl,
            p,
            avl,
            l,
            db,
            g,
        }
    }
}

impl Into<u64> for Descriptor {
    fn into(self) -> u64 {
        let base_low: u64 = ((self.base as u64) << Self::BASE_LOW_SHIFT_BEGIN) & Self::BASE_LOW_MASK;
        let base_high: u64 = (((self.base as u64) >> Self::BASE_LOW_LENGTH) << Self::BASE_HIGH_SHIFT_BEGIN) & Self::BASE_HIGH_MASK;
        let limit_low: u64 = ((self.limit as u64) << Self::LIMIT_LOW_SHIFT_BEGIN) & Self::LIMIT_LOW_MASK;
        let limit_high: u64 = (((self.limit as u64) >> Self::LIMIT_LOW_LENGTH) << Self::LIMIT_HIGH_SHIFT_BEGIN) & Self::LIMIT_HIGH_MASK;
        let segment_type: u64 = ((self.segment_type as u64) << Self::SEGMENT_TYPE_SHIFT_BEGIN) & Self::SEGMENT_TYPE_MASK;
        let s: u64 = if self.s {
            Self::S_MASK
        } else {
            0
        };
        let dpl: u64 = ((self.dpl as u64) << Self::DPL_SHIFT_BEGIN) & Self::DPL_MASK;
        let p: u64 = if self.p {
            Self::P_MASK
        } else {
            0
        };
        let avl: u64 = if self.avl {
            Self::AVL_MASK
        } else {
            0
        };
        let l: u64 = if self.l {
            Self::L_MASK
        } else {
            0
        };
        let db: u64 = if self.db {
            Self::DB_MASK
        } else {
            0
        };
        let g: u64 = if self.g {
            Self::G_MASK
        } else {
            0
        };
        base_low
        | base_high
        | limit_low
        | limit_high
        | segment_type
        | s
        | dpl
        | p
        | avl
        | l
        | db
        | g
    }
}

