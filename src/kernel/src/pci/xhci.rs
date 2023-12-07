mod capability;

use super::configuration::type_specific::base_address;

#[derive(Debug)]
pub struct Registers<'a> {
    base: usize,
    capability: &'a capability::Registers,
}

impl From<base_address::Address> for Registers<'_> {
    fn from(address: base_address::Address) -> Self {
        match address {
            base_address::Address::Memory64(address) => {
                let base: usize = address as usize;
                let capability: *const capability::Registers = base as *const capability::Registers;
                let capability: &capability::Registers = unsafe {
                    &*capability
                };
                Self {
                    base,
                    capability,
                }
            },
            _ => panic!("Can't get xHCI registers!"),
        }
    }
}

