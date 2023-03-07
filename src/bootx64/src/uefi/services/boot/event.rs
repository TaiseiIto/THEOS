// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7.1 Event, Timer and Task Priority Services

use {
    core::fmt,
    super::{
        protocol_handler,
        super::super::types::{
            status,
            void,
        },
    },
};

#[repr(C)]
pub struct CreateEvent(extern "efiapi" fn(u32, Tpl, EventNotify, &void::Void, &mut Event) -> status::Status);

impl fmt::Debug for CreateEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

pub type Event<'a> = &'a void::Void;
pub type EventNotify = extern "efiapi" fn(Event, &void::Void);

#[repr(C)]
pub struct CreateEventEx(extern "efiapi" fn(u32, Tpl, EventNotify, &void::Void, &protocol_handler::Guid, &mut Event) -> status::Status);

impl fmt::Debug for CreateEventEx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct CloseEvent(extern "efiapi" fn(Event) -> status::Status);

impl fmt::Debug for CloseEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct SignalEvent(extern "efiapi" fn(Event) -> status::Status);

impl fmt::Debug for SignalEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct WaitForEvent(extern "efiapi" fn(usize, &Event, &mut usize) -> status::Status);

impl fmt::Debug for WaitForEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct CheckEvent(extern "efiapi" fn(Event) -> status::Status);

impl fmt::Debug for CheckEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct SetTimer(extern "efiapi" fn(Event, TimerDelay, u64) -> status::Status);

impl fmt::Debug for SetTimer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[allow(dead_code)]
#[repr(C)]
pub enum TimerDelay {
    Cancel,
    Periodic,
    Relative,
}

#[repr(C)]
pub struct RaiseTpl(extern "efiapi" fn(Tpl) -> Tpl);

impl fmt::Debug for RaiseTpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

#[repr(C)]
pub struct RestoreTpl(extern "efiapi" fn(Tpl));

impl fmt::Debug for RestoreTpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0 as usize)
    }
}

pub type Tpl = usize;

