// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 4.6 EFI Configuration Table & Propaties Table

use {
    core::fmt,
    super::super::{
        services::boot::protocol_handler,
        types::void,
    },
};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Configuration<'a> {
    vendor_guid: protocol_handler::Guid,
    vendor_table: &'a void::Void,
}

#[derive(Clone)]
pub struct Configurations<'a> {
    configurations: &'a Configuration<'a>,
    number_of_tables: usize,
}

impl<'a> Configurations<'a> {
    pub fn new(configurations: &'a Configuration<'a>, number_of_tables: usize) -> Self {
        Self {
            configurations,
            number_of_tables,
        }
    }
}

impl<'a> fmt::Debug for Configurations<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_list()
            .entries(self.clone().into_iter())
            .finish()
    }
}

impl<'a> Iterator for Configurations<'a> {
    type Item = &'a Configuration<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.number_of_tables {
            0 => None,
            _ => {
                let configuration: &'a Configuration<'a> = self.configurations;
                let next_configuration = configuration as *const Configuration<'a>;
                let next_configuration = unsafe {
                    next_configuration.add(1)
                };
                let next_configuration: &'a Configuration<'a> = unsafe {
                    &*next_configuration
                };
                self.configurations = next_configuration;
                self.number_of_tables -= 1;
                Some(configuration)
            },
        }
    }
}

