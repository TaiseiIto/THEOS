extern crate alloc;

mod capability;
mod doorbell;
mod operational;
mod port;
mod runtime;
mod vtio;

use {
    alloc::vec::Vec,
    core::mem,
    super::configuration::type_specific::base_address,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Registers<'a> {
    base: usize,
    capability: &'a capability::Registers,
    operational: &'a operational::Registers,
    ports: Vec<&'a mut port::Registers>,
    runtime: &'a runtime::Registers,
    doorbells: Vec<&'a mut doorbell::Registers>,
    vtio: &'a vtio::Registers,
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
        let (operational, ports_base): (*const operational::Registers, usize) = (operational as *const operational::Registers, operational + 0x400);
        let operational: &operational::Registers = unsafe {
            &*operational
        };
        let max_ports: usize = capability.max_ports() as usize;
        let ports: Vec<&mut port::Registers> = (0..max_ports)
            .map(|port_number| {
                let port: usize = ports_base + port_number * mem::size_of::<port::Registers>();
                let port: *mut port::Registers = port as *mut port::Registers;
                unsafe {
                    &mut *port
                }
            })
            .collect();
        let runtime: usize = base + capability.runtime_register_space_offset() as usize;
        let runtime: *const runtime::Registers = runtime as *const runtime::Registers;
        let runtime: &runtime::Registers = unsafe {
            &*runtime
        };
        let doorbells: usize = base + capability.doorbell_array_offset() as usize;
        let doorbells: Vec<&mut doorbell::Registers> = (0..max_ports)
            .map(|port_number| {
                let doorbell: usize = doorbells + port_number * mem::size_of::<doorbell::Registers>();
                let doorbell: *mut doorbell::Registers = doorbell as *mut doorbell::Registers;
                unsafe {
                    &mut *doorbell
                }
            })
            .collect();
        let vtio: usize = base + capability.vtio_register_space_offset() as usize;
        let vtio: *const vtio::Registers = vtio as *const vtio::Registers;
        let vtio: &vtio::Registers = unsafe {
            &*vtio
        };
        Self {
            base,
            capability,
            operational,
            ports,
            runtime,
            doorbells,
            vtio,
        }
    }
}

