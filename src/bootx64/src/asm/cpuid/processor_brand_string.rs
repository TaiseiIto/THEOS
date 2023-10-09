use {
    alloc::{
        string::String,
        vec::Vec,
    },
    super::{
        CpuidOutRegisters,
        eax0x80000000::Eax0x80000000,
    },
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ProcessorBrandString {
    processor_brand_string: String,
}

impl ProcessorBrandString {
    pub fn new(eax0x80000000: &Eax0x80000000) -> Option<Self> {
        let first_eax: u32 = 0x80000002;
        let last_eax: u32 = 0x80000004;
        let ecx: u32 = 0x00000000;
        if last_eax <= eax0x80000000.max_eax() {
            let bytes: Vec<u8> = (first_eax..last_eax)
                .map(|eax| {
                    let CpuidOutRegisters {
                        eax,
                        ebx,
                        edx,
                        ecx,
                    } = CpuidOutRegisters::cpuid(eax, ecx);
                    [
                        eax,
                        ebx,
                        ecx,
                        edx,
                    ]
                        .into_iter()
                        .map(|register| register
                            .to_le_bytes()
                            .into_iter()
                            .filter(|byte| *byte != 0x00)
                        )
                        .flatten()
                })
                .flatten()
                .collect();
            match String::from_utf8(bytes) {
                Ok(processor_brand_string) => Some(Self {
                    processor_brand_string,
                }),
                _ => None,
            }
        } else {
            None
        }
    }
}

