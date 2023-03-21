// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

pub mod e_ident;
pub mod e_machine;
pub mod e_type;

use core::mem;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Header {
    e_ident: e_ident::Ident,
    e_type: e_type::Type,
    e_machine: e_machine::Machine,
    e_version: u32,
    e_entry: usize,
    e_phoff: usize,
}

const EI_NIDENT: usize = 0x10;
const E_TYPE_BEGIN: usize = EI_NIDENT;
const E_TYPE_LENGTH: usize = mem::size_of::<u16>();
const E_TYPE_END: usize = E_TYPE_BEGIN + E_TYPE_LENGTH;
const E_MACHINE_BEGIN: usize = E_TYPE_END;
const E_MACHINE_LENGTH: usize = mem::size_of::<u16>();
const E_MACHINE_END: usize = E_MACHINE_BEGIN + E_MACHINE_LENGTH;
const E_VERSION_BEGIN: usize = E_MACHINE_END;
const E_VERSION_LENGTH: usize = mem::size_of::<u32>();
const E_VERSION_END: usize = E_VERSION_BEGIN + E_VERSION_LENGTH;
const E_ENTRY_BEGIN: usize = E_VERSION_END;
const E_ENTRY_LENGTH: usize = mem::size_of::<usize>();
const E_ENTRY_END: usize = E_ENTRY_BEGIN + E_ENTRY_LENGTH;
const E_PHOFF_BEGIN: usize = E_ENTRY_END;
const E_PHOFF_LENGTH: usize = mem::size_of::<usize>();
const E_PHOFF_END: usize = E_PHOFF_BEGIN + E_PHOFF_LENGTH;

impl Header {
    pub fn new(header: &[u8]) -> Self {
        let e_ident: [u8; EI_NIDENT] = header[..EI_NIDENT]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_ident: e_ident::Ident = e_ident.into();
        let e_type: [u8; E_TYPE_LENGTH] = header[E_TYPE_BEGIN..E_TYPE_END]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_type = u16::from_le_bytes(e_type);
        let e_type: e_type::Type = e_type.into();
        let e_machine: [u8; E_MACHINE_LENGTH] = header[E_MACHINE_BEGIN..E_MACHINE_END]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_machine = u16::from_le_bytes(e_machine);
        let e_machine: e_machine::Machine = e_machine.into();
        let e_version: [u8; E_VERSION_LENGTH] = header[E_VERSION_BEGIN..E_VERSION_END]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_version = u32::from_le_bytes(e_version);
        let e_entry: [u8; E_ENTRY_LENGTH] = header[E_ENTRY_BEGIN..E_ENTRY_END]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_entry = usize::from_le_bytes(e_entry);
        let e_phoff: [u8; E_PHOFF_LENGTH] = header[E_PHOFF_BEGIN..E_PHOFF_END]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_phoff = usize::from_le_bytes(e_phoff);
        Self {
            e_ident,
            e_type,
            e_machine,
            e_version,
            e_entry,
            e_phoff,
        }
    }
}

