use {
    std::mem,
    super::super::{
        binary::Binary,
        guid,
    },
};

#[derive(Clone, Copy, Debug)]
pub struct OemParameters {
    parameters: [OemParameter; 0xa],
    size: usize,
}

impl OemParameters {
    pub fn null(size: usize) -> Self {
        let parameters = [OemParameter::null(); 0xa];
        Self {
            parameters,
            size,
        }
    }
}

impl Binary for OemParameters {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.parameters
            .iter()
            .map(|parameter| parameter.to_bytes().into_iter())
            .flatten()
            .collect();
        bytes.resize(self.size, 0x00);
        bytes
    }
}

#[derive(Clone, Copy, Debug)]
struct OemParameter {
    parameter_guid: guid::Guid,
    custom_defined: [u8; 0x20],
}

impl OemParameter {
    fn null() -> Self {
        let parameter_guid = guid::Guid::null();
        let custom_defined = [0u8; 0x20];
        Self {
            parameter_guid,
            custom_defined,
        }
    }
}

impl Binary for OemParameter {
    fn to_bytes(&self) -> Vec<u8> {
        let parameter_guid: u128 = self.parameter_guid.to_u128();
        let parameter_guid: [u8; 0x10] = unsafe {
            mem::transmute::<u128, [u8; 0x10]>(parameter_guid)
        };
        let parameter_guid: Vec<u8> = parameter_guid.to_vec();
        let mut custom_defined: Vec<u8> = self.custom_defined.to_vec();
        let mut bytes: Vec<u8> = parameter_guid;
        bytes.append(&mut custom_defined);
        bytes
    }
}

