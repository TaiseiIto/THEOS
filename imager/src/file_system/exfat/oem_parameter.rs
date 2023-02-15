use {
    std::{
        fmt,
        mem,
    },
    super::super::super::guid,
};

const NUM_OF_OEM_PARAMETERS: usize = 0xa;

#[derive(Clone, Copy, Debug)]
pub struct OemParameters {
    parameters: [OemParameter; NUM_OF_OEM_PARAMETERS],
    size: usize,
}

impl OemParameters {
    pub fn null(size: usize) -> Self {
        let parameters = [OemParameter::null(); NUM_OF_OEM_PARAMETERS];
        Self {
            parameters,
            size,
        }
    }
}

impl From<&Vec<u8>> for OemParameters {
    fn from(bytes: &Vec<u8>) -> Self {
        let size: usize = bytes.len();
        let oem_parameter_size: usize = guid::GUID_SIZE + CUSTOM_DEFINED_SIZE;
        let parameters: Vec<Vec<u8>> = bytes
            .chunks(oem_parameter_size)
            .map(|parameter| parameter.to_vec())
            .collect();
        let parameters: Vec<OemParameter> = parameters[..NUM_OF_OEM_PARAMETERS]
            .into_iter()
            .map(|parameter| OemParameter::from(parameter))
            .collect();
        let parameters: [OemParameter; NUM_OF_OEM_PARAMETERS] = parameters
            .try_into()
            .expect("Can't read OEM parameters.");
        Self {
            parameters,
            size,
        }
    }
}

impl Into<Vec<u8>> for &OemParameters {
    fn into(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = self.parameters
            .iter()
            .map(|parameter| Into::<Vec<u8>>::into(parameter).into_iter())
            .flatten()
            .collect();
        bytes.resize(self.size, 0x00);
        bytes
    }
}

impl fmt::Display for OemParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters: String = self.parameters
            .iter()
            .enumerate()
            .map(|(i, parameter)| format!("{}", parameter)
                .lines()
                .map(|line| format!("parameters[{}].{}", i, line))
                .collect::<Vec<String>>()
                .join("\n"))
            .fold(String::new(), |parameters, parameter| parameters + &parameter);
        let size: String = format!("size: {:#x}", self.size);
        let oem_parameters: String = format!("{}{}", parameters, size);
        write!(f, "{}", oem_parameters)
    }
}

const CUSTOM_DEFINED_SIZE: usize = 0x20;

#[derive(Clone, Copy, Debug)]
struct OemParameter {
    parameter_guid: guid::Guid,
    custom_defined: [u8; CUSTOM_DEFINED_SIZE],
}

impl OemParameter {
    fn null() -> Self {
        let parameter_guid = guid::Guid::null();
        let custom_defined = [0u8; CUSTOM_DEFINED_SIZE];
        Self {
            parameter_guid,
            custom_defined,
        }
    }
}

impl From<&Vec<u8>> for OemParameter {
    fn from(bytes: &Vec<u8>) -> Self {
        let bytes: Vec<u8> = bytes.clone();
        let (parameter_guid, bytes): (&[u8], &[u8]) = bytes
            .split_at(guid::GUID_SIZE);
        let parameter_guid: [u8; guid::GUID_SIZE] = parameter_guid
            .try_into()
            .expect("Can't read an OEM parameter.");
        let parameter_guid: u128 = unsafe {
            mem::transmute::<[u8; guid::GUID_SIZE], u128>(parameter_guid)
        };
        let parameter_guid = guid::Guid::read(parameter_guid);
        let bytes: Vec<u8> = bytes.to_vec();
        let (custom_defined, _): (&[u8], &[u8]) = bytes.split_at(CUSTOM_DEFINED_SIZE);
        let custom_defined: [u8; CUSTOM_DEFINED_SIZE] = custom_defined
            .try_into()
            .expect("Can't read an OEM parameter.");
        Self {
            parameter_guid,
            custom_defined,
        }
    }
}

impl Into<Vec<u8>> for &OemParameter {
    fn into(self) -> Vec<u8> {
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

impl fmt::Display for OemParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameter_guid: String = format!("{}", self.parameter_guid)
            .lines()
            .map(|line| format!("parameter_guid.{}", line))
            .collect::<Vec<String>>()
            .join("\n");
        let custom_defined: String = self.custom_defined
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join(" ");
        let custom_defined: String = custom_defined[0..custom_defined.len() - 1].to_string();
        let custom_defined: String = format!("custom_defined: {}", custom_defined);
        let oem_parameter: String = format!("{}{}", parameter_guid, custom_defined);
        write!(f, "{}", oem_parameter)
    }
}

