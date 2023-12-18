extern crate alloc;

use {
    alloc::vec::Vec,
    super::CONFIGURATION_SIZE,
};

#[derive(Clone, Debug)]
pub struct Structure {
    capability_id: u8,
    next_capability_pointer: u8,
}

impl Structure {
    pub fn get_all(configuration: &[u8; CONFIGURATION_SIZE], first_capability_pointer: u8) -> Vec<Self> {
        StructureIterator::new(configuration, first_capability_pointer).collect()
    }

    fn get_one(configuration: &[u8; CONFIGURATION_SIZE], capability_pointer: u8) -> Option<Self> {
        match capability_pointer {
            0x00 => None,
            capability_pointer => {
                let capability_pointer: usize = capability_pointer as usize;
                let capability_id: u8 = configuration[capability_pointer];
                let next_capability_pointer: u8 = configuration[capability_pointer + 1];
                Some(Self {
                    capability_id,
                    next_capability_pointer,
                })
            },
        }
    }
}

#[derive(Clone)]
struct StructureIterator<'a> {
    configuration: &'a [u8; CONFIGURATION_SIZE],
    structure: Option<Structure>,
}

impl<'a> StructureIterator<'a> {
    fn new(configuration: &'a [u8; CONFIGURATION_SIZE], first_capability_pointer: u8) -> Self {
        let structure: Option<Structure> = Structure::get_one(configuration, first_capability_pointer);
        Self {
            configuration,
            structure,
        }
    }
}

impl<'a> Iterator for StructureIterator<'a> {
    type Item = Structure;

    fn next(&mut self) -> Option<Self::Item> {
        let StructureIterator {
            configuration,
            structure,
        } = self.clone();
        if let Some(structure) = &structure {
            let next_capability_pointer: u8 = structure.next_capability_pointer;
            self.structure = Structure::get_one(configuration, next_capability_pointer);
        }
        structure
    }
}

