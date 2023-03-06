// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 8.3 Time Services

use {
    core::fmt,
    super::super::super::types::status,
};

pub struct GetTime(extern "efiapi" fn(&Time, &TimeCapabilities) -> status::Status);

impl fmt::Debug for GetTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

pub struct SetTime(extern "efiapi" fn(&Time) -> status::Status);

impl fmt::Debug for SetTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

pub struct GetWakeupTime(extern "efiapi" fn(&bool, &bool, &Time) -> status::Status);

impl fmt::Debug for GetWakeupTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

pub struct SetWakeupTime(extern "efiapi" fn(bool, &Time) -> status::Status);

impl fmt::Debug for SetWakeupTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Time {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    pad1: u8,
    nanosecond: u32,
    time_zone: i16,
    day_light: u8,
    pad2: u8,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct TimeCapabilities {
    resolution: u32,
    accuracy: u32,
    sets_to_zero: bool,
}

