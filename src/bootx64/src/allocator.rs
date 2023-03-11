use super::uefi::tables::system;

pub struct Allocator {
}

impl Allocator {
    pub fn set_system(system: system::System<'static>) {
        unsafe {
            SYSTEM = Some(system);
        }
    }
}

static mut SYSTEM: Option<system::System> = None;

