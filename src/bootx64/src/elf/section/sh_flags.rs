// References
// https://refspecs.linuxfoundation.org/elf/elf.pdf
// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

#[allow(dead_code)]
#[derive(Debug)]
pub struct Flags {
    write: bool,
    alloc: bool,
    execinstr: bool,
    merge: bool,
    strings: bool,
    info_link: bool,
    link_order: bool,
    os_nonconforming: bool,
    group: bool,
    tls: bool,
    maskos: u8,
    maskproc: u8,
    ordered: bool,
    exclude: bool,
}

impl From<usize> for Flags {
    fn from(sh_flags: usize) -> Self {
        let write: bool = sh_flags & 0x00000001 != 0;
        let alloc: bool = sh_flags & 0x00000002 != 0;
        let execinstr: bool = sh_flags & 0x00000004 != 0;
        let merge: bool = sh_flags & 0x00000010 != 0;
        let strings: bool = sh_flags & 0x00000020 != 0;
        let info_link: bool = sh_flags & 0x00000040 != 0;
        let link_order: bool = sh_flags & 0x00000080 != 0;
        let os_nonconforming: bool = sh_flags & 0x00000100 != 0;
        let group: bool = sh_flags & 0x00000200 != 0;
        let tls: bool = sh_flags & 0x00000400 != 0;
        let maskos: u8 = (sh_flags >> 20) as u8;
        let maskproc: u8 = (sh_flags >> 28) as u8;
        let ordered: bool = sh_flags & 0x04000000 != 0;
        let exclude: bool = sh_flags & 0x08000000 != 0;
        Self {
            write,
            alloc,
            execinstr,
            merge,
            strings,
            info_link,
            link_order,
            os_nonconforming,
            group,
            tls,
            maskos,
            maskproc,
            ordered,
            exclude,
        }
    }
}

