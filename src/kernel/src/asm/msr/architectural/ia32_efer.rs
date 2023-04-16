#[allow(dead_code)]
#[derive(Debug)]
pub struct Ia32Efer {
    sce: bool,
    lme: bool,
    lma: bool,
    nxe: bool,
}

impl Ia32Efer {
    pub fn lme(&self) -> bool {
        self.lme
    }
}

