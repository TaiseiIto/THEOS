use super::super::types::status;

// References
// https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf
// 12.3 Simple Text Input Protocol
pub struct SimpleTextInput {
    reset: InputReset,
}

type InputReset = fn(*const SimpleTextInput, bool) -> status::Status;

