use std::{
    fmt,
    mem,
};
use super::{
    Sector,
    Packable,
    Unpackable,
};

#[derive(Clone, Copy, Debug)]
pub struct OemParameterSector {
    parameters: [OemParameter; 0xa],
    reserved: [u8; 0x20],
}

impl OemParameterSector {
    pub fn null_parameters() -> Self {
        Self {
            parameters: [OemParameter::null_parameter(); 0xa],
            reserved: [0; 0x20],
        }
    }
}

impl Packable for OemParameterSector {
    type Packed = PackedOemParameterSector;

    fn pack(&self) -> Self::Packed {
        Self::Packed {
            parameters: self.parameters.map(|parameter| parameter.pack()),
            reserved: self.reserved,
        }
    }
}

impl Sector for OemParameterSector {
    fn to_bytes(&self) -> super::RawSector {
        self.pack().to_bytes()
    }
}

impl fmt::Display for OemParameterSector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, parameter) in self.parameters.iter().enumerate() {
            let parameter = format!("{}", parameter);
            let parameter = parameter.replace("oem_parameter", &format!("opem_parameters.oem_parameter[{}]", i));
            write!(f, "{}\n", parameter)?;
        }
        write!(f, "oem_parameter_sector.reserved = {:x?}", self.reserved)
    }
}

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct PackedOemParameterSector {
    parameters: [PackedOemParameter; 0xa],
    reserved: [u8; 0x20],
}

impl Unpackable for PackedOemParameterSector {
    type Unpacked = OemParameterSector;

    fn unpack(&self) -> Self::Unpacked {
        Self::Unpacked {
            parameters: self.parameters.map(|parameter| parameter.unpack()),
            reserved: self.reserved,
        }
    }
}

impl Sector for PackedOemParameterSector {
    fn to_bytes(&self) -> super::RawSector {
        unsafe {
            mem::transmute::<Self, [u8; mem::size_of::<Self>()]>(*self)
        }
    }
}

impl fmt::Display for PackedOemParameterSector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)
    }
}

#[derive(Clone, Copy, Debug)]
struct OemParameter {
    parameters_guid: [u8; 0x10],
    custom_defined: [u8; 0x20],
}

impl OemParameter {
    fn null_parameter() -> Self {
        Self {
            parameters_guid: [0; 0x10],
            custom_defined: [0; 0x20],
        }
    }
}

impl Packable for OemParameter {
    type Packed = PackedOemParameter;

    fn pack(&self) -> Self::Packed {
        Self::Packed {
            parameters_guid: self.parameters_guid,
            custom_defined: self.custom_defined,
        }
    }
}

impl fmt::Display for OemParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "oem_parameter.parameters_guid = {:x?}\n", self.parameters_guid)?;
        write!(f, "oem_parameter.custom_defined = {:x?}", self.custom_defined)
    }
}

#[derive(Clone, Copy)]
#[repr(packed)]
struct PackedOemParameter {
    parameters_guid: [u8; 0x10],
    custom_defined: [u8; 0x20],
}

impl Unpackable for PackedOemParameter {
    type Unpacked = OemParameter;

    fn unpack(&self) -> Self::Unpacked {
        Self::Unpacked {
            parameters_guid: self.parameters_guid,
            custom_defined: self.custom_defined,
        }
    }
}

impl fmt::Display for PackedOemParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.unpack().fmt(f)
    }
}


