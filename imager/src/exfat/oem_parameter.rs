use super::super::guid;

#[derive(Clone, Copy, Debug)]
pub struct OemParameters {
    parameters: [OemParameter; 0xa],
}

impl OemParameters {
    pub fn null() -> Self {
        let parameters = [OemParameter::null(); 0xa];
        Self {
            parameters,
        }
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

