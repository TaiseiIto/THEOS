use super::super::super::cpuid;

#[derive(Debug)]
pub struct Ia32Efer {
}

impl Ia32Efer {
    pub fn get(cpuid: &Option<cpuid::Cpuid>) -> Option<Self> {
        match cpuid {
            Some(cpuid) => if cpuid.supports_ia32_efer() {
                Some(Self {
                })
            } else {
                None
            },
            None => None,
        }
    }
}

