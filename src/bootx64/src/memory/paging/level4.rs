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
    writable: bool,
    user_mode_access: bool,
    page_write_through: bool,
    page_cache_disable: bool,
    accessed: bool,
    restart: bool,
    execute_disable: bool,
}

impl PageMapLevel4Entry {
    const PRESENT_SHIFT: usize = 0;
    const WRITABLE_SHIFT: usize = 1;
    const USER_MODE_ACCESS_SHIFT: usize = 2;
    const PAGE_WRITE_THROUGH_SHIFT: usize = 3;
    const PAGE_CACHE_DISABLE_SHIFT: usize = 4;
    const ACCESSED_SHIFT: usize = 5;
    const RESTART_SHIFT: usize = 11;
    const EXECUTE_DISABLE_SHIFT: usize = 63;

    const PRESENT_MASK: u64 = 1 << Self::PRESENT_SHIFT;
    const WRITABLE_MASK: u64 = 1 << Self::WRITABLE_SHIFT;
    const USER_MODE_ACCESS_MASK: u64 = 1 << Self::USER_MODE_ACCESS_SHIFT;
    const PAGE_WRITE_THROUGH_MASK: u64 = 1 << Self::PAGE_WRITE_THROUGH_SHIFT;
    const PAGE_CACHE_DISABLE_MASK: u64 = 1 << Self::PAGE_CACHE_DISABLE_SHIFT;
    const ACCESSED_MASK: u64 = 1 << Self::ACCESSED_SHIFT;
    const RESTART_MASK: u64 = 1 << Self::RESTART_SHIFT;
    const EXECUTE_DISABLE_MASK: u64 = 1 << Self::EXECUTE_DISABLE_SHIFT;

    fn read(page_map_level_4_entry: u64) -> Option<Self> {
        if page_map_level_4_entry & Self::PRESENT_MASK != 0 {
            let writable: bool = page_map_level_4_entry & Self::WRITABLE_MASK != 0;
            let user_mode_access: bool = page_map_level_4_entry & Self::USER_MODE_ACCESS_MASK != 0;
            let page_write_through: bool = page_map_level_4_entry & Self::PAGE_WRITE_THROUGH_MASK != 0;
            let page_cache_disable: bool = page_map_level_4_entry & Self::PAGE_CACHE_DISABLE_MASK != 0;
            let accessed: bool = page_map_level_4_entry & Self::ACCESSED_MASK != 0;
            let restart: bool = page_map_level_4_entry & Self::RESTART_MASK != 0;
            let execute_disable: bool = page_map_level_4_entry & Self::EXECUTE_DISABLE_MASK != 0;
            Some(Self {
                writable,
                user_mode_access,
                page_write_through,
                page_cache_disable,
                accessed,
                restart,
                execute_disable,
            })
        } else {
            None
        }
    }
}

