use {
    core::fmt,
    super::super::types::status,
};

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 12.4 Simple Text Output Protocol
pub struct SimpleTextOutput {
    reset: TextReset,
}

impl fmt::Debug for SimpleTextOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reset: usize = self.reset as usize;
        write!(f, "SimpleTextOutput {{\n").expect("Can't print a simple text output protocol!");
        write!(f, "    reset: {:#x},\n", reset).expect("Can't print a simple text output protocol!");
        write!(f, "}}")
    }
}

type TextReset = fn(&SimpleTextOutput, bool) -> status::Status;

