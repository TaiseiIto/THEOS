// References
// Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 3 System Programming Guide, Chapter 4 Paging, Section 5 4-Level Paging And 5-Level Paging

use alloc::vec::Vec;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Cr3<'a> {
    pwt: bool,
    pcd: bool,
    page_map_level_4_table: &'a [u64; 0x200],
    page_map_level_4_entries: Vec<PageMapLevel4Entry>,
}

impl Cr3<'_> {
    const PWT_SHIFT: usize = 3;
    const PCD_SHIFT: usize = 4;

    const PWT_MASK: u64 = 1 << Self::PWT_SHIFT;
    const PCD_MASK: u64 = 1 << Self::PCD_SHIFT;
    const PAGE_DIRECTORY_BASE_MASK: u64 = 0xfffffffffffff000;
}

impl From<u64> for Cr3<'_> {
	fn from(cr3: u64) -> Self {
        let pwt: bool = cr3 & Self::PWT_MASK != 0;
        let pcd: bool = cr3 & Self::PCD_MASK != 0;
        let page_map_level_4_table: u64 = cr3 & Self::PAGE_DIRECTORY_BASE_MASK;
        let page_map_level_4_table: *const [u64; 0x200] = page_map_level_4_table as *const [u64; 0x200];
        let page_map_level_4_table: &[u64; 0x200] = unsafe {
            &*page_map_level_4_table
        };
        let page_map_level_4_entries: Vec<PageMapLevel4Entry> = page_map_level_4_table
            .iter()
            .filter_map(|page_map_level_4_entry| PageMapLevel4Entry::read(*page_map_level_4_entry))
            .collect();
        Self {
            pwt,
            pcd,
            page_map_level_4_table,
            page_map_level_4_entries,
        }
	}
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PageMapLevel4Entry {
}

impl PageMapLevel4Entry {
    const PRESENT_SHIFT: usize = 0;

    const PRESENT_MASK: u64 = 1 << Self::PRESENT_SHIFT;

    fn read(page_map_level_4_entry: u64) -> Option<Self> {
        if page_map_level_4_entry & Self::PRESENT_MASK != 0 {
            Some(Self {
            })
        } else {
            None
        }
    }
}

