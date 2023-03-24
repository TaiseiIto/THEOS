// References
// Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 3 System Programming Guide, Chapter 4 Paging, Section 5 4-Level Paging And 5-Level Paging

#[allow(dead_code)]
#[derive(Debug)]
pub struct Cr3 {
    pwt: bool,
    pcd: bool,
    page_directory_base: u64,
}

impl Cr3 {
    const PWT_SHIFT: usize = 3;
    const PCD_SHIFT: usize = 4;

    const PWT_MASK: u64 = 1 << Self::PWT_SHIFT;
    const PCD_MASK: u64 = 1 << Self::PCD_SHIFT;
    const PAGE_DIRECTORY_BASE_MASK: u64 = 0xfffffffffffff000;
}

impl From<u64> for Cr3 {
	fn from(cr3: u64) -> Self {
        let pwt: bool = cr3 & Self::PWT_MASK != 0;
        let pcd: bool = cr3 & Self::PCD_MASK != 0;
        let page_directory_base: u64 = cr3 & Self::PAGE_DIRECTORY_BASE_MASK;
        Self {
            pwt,
            pcd,
            page_directory_base,
        }
	}
}

