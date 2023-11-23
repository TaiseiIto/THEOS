use super::asm;

#[derive(Debug)]
pub struct ConfigurationAddress {
    bus: u8,
    device: u8,
    function: u8,
}

impl ConfigurationAddress {
    pub fn new(bus: u8, device: u8, function: u8) -> Self {
        Self {
            bus,
            device,
            function,
        }
    }
}
