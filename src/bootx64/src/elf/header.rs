// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

pub mod e_ident;
pub mod e_machine;
pub mod e_type;

use {
    core::{
        arch::asm,
        mem,
    },
    crate::{
        serial_print,
        serial_println,
    },
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Header {
    e_ident: e_ident::Ident,
    e_type: e_type::Type,
    e_machine: e_machine::Machine,
    e_version: u32,
    e_entry: usize,
    e_phoff: usize,
    e_shoff: usize,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
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
const E_SHOFF_BEGIN: usize = E_PHOFF_END;
const E_SHOFF_LENGTH: usize = mem::size_of::<usize>();
const E_SHOFF_END: usize = E_SHOFF_BEGIN + E_SHOFF_LENGTH;
const E_FLAGS_BEGIN: usize = E_SHOFF_END;
const E_FLAGS_LENGTH: usize = mem::size_of::<u32>();
const E_FLAGS_END: usize = E_FLAGS_BEGIN + E_FLAGS_LENGTH;
const E_EHSIZE_BEGIN: usize = E_FLAGS_END;
const E_EHSIZE_LENGTH: usize = mem::size_of::<u16>();
const E_EHSIZE_END: usize = E_EHSIZE_BEGIN + E_EHSIZE_LENGTH;
const E_PHENTSIZE_BEGIN: usize = E_EHSIZE_END;
const E_PHENTSIZE_LENGTH: usize = mem::size_of::<u16>();
const E_PHENTSIZE_END: usize = E_PHENTSIZE_BEGIN + E_PHENTSIZE_LENGTH;
const E_PHNUM_BEGIN: usize = E_PHENTSIZE_END;
const E_PHNUM_LENGTH: usize = mem::size_of::<u16>();
const E_PHNUM_END: usize = E_PHNUM_BEGIN + E_PHNUM_LENGTH;
const E_SHENTSIZE_BEGIN: usize = E_PHNUM_END;
const E_SHENTSIZE_LENGTH: usize = mem::size_of::<u16>();
const E_SHENTSIZE_END: usize = E_SHENTSIZE_BEGIN + E_SHENTSIZE_LENGTH;
const E_SHNUM_BEGIN: usize = E_SHENTSIZE_END;
const E_SHNUM_LENGTH: usize = mem::size_of::<u16>();
const E_SHNUM_END: usize = E_SHNUM_BEGIN + E_SHNUM_LENGTH;
const E_SHSTRNDX_BEGIN: usize = E_SHNUM_END;
const E_SHSTRNDX_LENGTH: usize = mem::size_of::<u16>();
const E_SHSTRNDX_END: usize = E_SHSTRNDX_BEGIN + E_SHSTRNDX_LENGTH;

impl Header {
    pub fn run(&self) {
        serial_println!("Header.run()");
        serial_println!("self.e_entry = {:#x}", self.e_entry);
        unsafe {
            asm!(
                "xor rsp, rsp",
                "call rax",
                in("rax") self.e_entry,
            );
        }
    }

    pub fn e_phentsize(&self) -> usize {
        self.e_phentsize as usize
    }

    pub fn e_phnum(&self) -> usize {
        self.e_phnum as usize
    }

    pub fn e_phoff(&self) -> usize {
        self.e_phoff
    }

    pub fn e_shentsize(&self) -> usize {
        self.e_shentsize as usize
    }

    pub fn e_shnum(&self) -> usize {
        self.e_shnum as usize
    }

    pub fn e_shoff(&self) -> usize {
        self.e_shoff
    }

    pub fn e_shstrndx(&self) -> usize {
        self.e_shstrndx as usize
    }
}

impl From<&[u8]> for Header {
    fn from(header: &[u8]) -> Self {
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
        let e_shoff: [u8; E_SHOFF_LENGTH] = header[E_SHOFF_BEGIN..E_SHOFF_END]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_shoff = usize::from_le_bytes(e_shoff);
        let e_flags: [u8; E_FLAGS_LENGTH] = header[E_FLAGS_BEGIN..E_FLAGS_END]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_flags = u32::from_le_bytes(e_flags);
        let e_ehsize: [u8; E_EHSIZE_LENGTH] = header[E_EHSIZE_BEGIN..E_EHSIZE_END]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_ehsize = u16::from_le_bytes(e_ehsize);
        let e_phentsize: [u8; E_PHENTSIZE_LENGTH] = header[E_PHENTSIZE_BEGIN..E_PHENTSIZE_END]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_phentsize = u16::from_le_bytes(e_phentsize);
        let e_phnum: [u8; E_PHNUM_LENGTH] = header[E_PHNUM_BEGIN..E_PHNUM_END]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_phnum = u16::from_le_bytes(e_phnum);
        let e_shentsize: [u8; E_SHENTSIZE_LENGTH] = header[E_SHENTSIZE_BEGIN..E_SHENTSIZE_END]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_shentsize = u16::from_le_bytes(e_shentsize);
        let e_shnum: [u8; E_SHNUM_LENGTH] = header[E_SHNUM_BEGIN..E_SHNUM_END]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_shnum = u16::from_le_bytes(e_shnum);
        let e_shstrndx: [u8; E_SHSTRNDX_LENGTH] = header[E_SHSTRNDX_BEGIN..E_SHSTRNDX_END]
            .try_into()
            .expect("Can't read an ELF header!");
        let e_shstrndx = u16::from_le_bytes(e_shstrndx);
        Self {
            e_ident,
            e_type,
            e_machine,
            e_version,
            e_entry,
            e_phoff,
            e_shoff,
            e_flags,
            e_ehsize,
            e_phentsize,
            e_phnum,
            e_shentsize,
            e_shnum,
            e_shstrndx,
        }
    }
}

