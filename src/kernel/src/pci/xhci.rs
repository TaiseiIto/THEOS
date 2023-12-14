mod capability;
mod operational;

use super::configuration::type_specific::base_address;

#[derive(Debug)]
pub struct Registers<'a> {
    base: usize,
    capability: &'a capability::Registers,
    operational: &'a operational::Registers,
}

impl From<base_address::Address> for Registers<'_> {
    fn from(address: base_address::Address) -> Self {
        let base: usize = match address {
            base_address::Address::Memory64(address) => {
                address as usize
            },
            base_address::Address::Memory32(address) => {
                address as usize
            },
            base_address::Address::IO(_) => panic!("Can't get xHCI registers!"),
        };
        let capability: *const capability::Registers = base as *const capability::Registers;
        let capability: &capability::Registers = unsafe {
            &*capability
        };
        let caplength: usize = capability.caplength() as usize;
        let operational: usize = base + caplength;
        let operational: *const operational::Registers = operational as *const operational::Registers;
        let operational: &operational::Registers = unsafe {
            &*operational
        };
        Self {
            base,
            capability,
            operational,
        }
    }
}

