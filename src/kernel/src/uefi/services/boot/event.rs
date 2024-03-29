// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 7.1 Event, Timer and Task Priority Services

use {
    super::{
        protocol_handler,
        super::super::types::{
            status,
            void,
        },
    },
    wrapped_function::WrappedFunction,
};

#[derive(WrappedFunction)]
#[repr(C)]
pub struct CreateEvent(pub extern "efiapi" fn(u32, Tpl, EventNotify, &void::Void, &mut Event) -> status::Status);

pub type Event<'a> = &'a void::Void;
pub type EventNotify = extern "efiapi" fn(Event, &void::Void);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct CreateEventEx(pub extern "efiapi" fn(u32, Tpl, EventNotify, &void::Void, &protocol_handler::Guid, &mut Event) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct CloseEvent(pub extern "efiapi" fn(Event) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct SignalEvent(pub extern "efiapi" fn(Event) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct WaitForEvent(pub extern "efiapi" fn(usize, &Event, &mut usize) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct CheckEvent(pub extern "efiapi" fn(Event) -> status::Status);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct SetTimer(pub extern "efiapi" fn(Event, TimerDelay, u64) -> status::Status);

#[allow(dead_code)]
#[repr(C)]
pub enum TimerDelay {
    Cancel,
    Periodic,
    Relative,
}

#[derive(WrappedFunction)]
#[repr(C)]
pub struct RaiseTpl(pub extern "efiapi" fn(Tpl) -> Tpl);

#[derive(WrappedFunction)]
#[repr(C)]
pub struct RestoreTpl(pub extern "efiapi" fn(Tpl));

pub type Tpl = usize;

