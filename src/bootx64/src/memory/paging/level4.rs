// References
// Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 3 System Programming Guide, Chapter 4 Paging, Section 5 4-Level Paging And 5-Level Paging

use alloc::vec::Vec;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Cr3<'a> {
    pwt: bool,
    pcd: bool,
    page_map_level_4_table: &'a [u64; ENTRIES],
    page_map_level_4_entries: Vec<PageMapLevel4Entry<'a>>,
}

const ENTRIES: usize = 0x200;

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
        let page_map_level_4_table: *const [u64; ENTRIES] = page_map_level_4_table as *const [u64; ENTRIES];
        let page_map_level_4_table: &[u64; ENTRIES] = unsafe {
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
pub struct PageMapLevel4Entry<'a> {
    writable: bool,
    user_mode_access: bool,
    page_write_through: bool,
    page_cache_disable: bool,
    accessed: bool,
    restart: bool,
    execute_disable: bool,
    page_directory_pointer_table: &'a [u64; ENTRIES],
    page_directory_pointer_entries: Vec<PageDirectoryPointerEntry<'a>>,
}

impl PageMapLevel4Entry<'_> {
    const PRESENT_SHIFT: usize = 0;
    const WRITABLE_SHIFT: usize = 1;
    const USER_MODE_ACCESS_SHIFT: usize = 2;
    const PAGE_WRITE_THROUGH_SHIFT: usize = 3;
    const PAGE_CACHE_DISABLE_SHIFT: usize = 4;
    const ACCESSED_SHIFT: usize = 5;
    const RESTART_SHIFT: usize = 11;
    const EXECUTE_DISABLE_SHIFT: usize = 63;
    const PAGE_DIRECTORY_POINTER_TABLE_SHIFT_BEGIN: usize = 12;
    const PAGE_DIRECTORY_POINTER_TABLE_SHIFT_END: usize = 52;

    const PRESENT_MASK: u64 = 1 << Self::PRESENT_SHIFT;
    const WRITABLE_MASK: u64 = 1 << Self::WRITABLE_SHIFT;
    const USER_MODE_ACCESS_MASK: u64 = 1 << Self::USER_MODE_ACCESS_SHIFT;
    const PAGE_WRITE_THROUGH_MASK: u64 = 1 << Self::PAGE_WRITE_THROUGH_SHIFT;
    const PAGE_CACHE_DISABLE_MASK: u64 = 1 << Self::PAGE_CACHE_DISABLE_SHIFT;
    const ACCESSED_MASK: u64 = 1 << Self::ACCESSED_SHIFT;
    const RESTART_MASK: u64 = 1 << Self::RESTART_SHIFT;
    const EXECUTE_DISABLE_MASK: u64 = 1 << Self::EXECUTE_DISABLE_SHIFT;
    const PAGE_DIRECTORY_POINTER_TABLE_MASK: u64 = (1 << Self::PAGE_DIRECTORY_POINTER_TABLE_SHIFT_END) - (1 << Self::PAGE_DIRECTORY_POINTER_TABLE_SHIFT_BEGIN);

    fn read(page_map_level_4_entry: u64) -> Option<Self> {
        if page_map_level_4_entry & Self::PRESENT_MASK != 0 {
            let writable: bool = page_map_level_4_entry & Self::WRITABLE_MASK != 0;
            let user_mode_access: bool = page_map_level_4_entry & Self::USER_MODE_ACCESS_MASK != 0;
            let page_write_through: bool = page_map_level_4_entry & Self::PAGE_WRITE_THROUGH_MASK != 0;
            let page_cache_disable: bool = page_map_level_4_entry & Self::PAGE_CACHE_DISABLE_MASK != 0;
            let accessed: bool = page_map_level_4_entry & Self::ACCESSED_MASK != 0;
            let restart: bool = page_map_level_4_entry & Self::RESTART_MASK != 0;
            let execute_disable: bool = page_map_level_4_entry & Self::EXECUTE_DISABLE_MASK != 0;
            let page_directory_pointer_table: u64 = page_map_level_4_entry & Self::PAGE_DIRECTORY_POINTER_TABLE_MASK;
            let page_directory_pointer_table: *const [u64; ENTRIES] = page_directory_pointer_table as *const [u64; ENTRIES];
            let page_directory_pointer_table: &[u64; ENTRIES] = unsafe {
                &*page_directory_pointer_table
            };
            let page_directory_pointer_entries: Vec<PageDirectoryPointerEntry> = page_directory_pointer_table
                .iter()
                .filter_map(|page_directory_pointer_entry| PageDirectoryPointerEntry::read(*page_directory_pointer_entry))
                .collect();
            Some(Self {
                writable,
                user_mode_access,
                page_write_through,
                page_cache_disable,
                accessed,
                restart,
                execute_disable,
                page_directory_pointer_table,
                page_directory_pointer_entries,
            })
        } else {
            None
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PageDirectoryPointerEntry<'a> {
    writable: bool,
    user_mode_access: bool,
    page_write_through: bool,
    page_cache_disable: bool,
    accessed: bool,
    dirty: bool,
    page_size_1_gib: bool,
    restart: bool,
    execute_disable: bool,
    page_directory_table: Option<&'a [u64; ENTRIES]>,
}

impl PageDirectoryPointerEntry<'_> {
    const PRESENT_SHIFT: usize = 0;
    const WRITABLE_SHIFT: usize = 1;
    const USER_MODE_ACCESS_SHIFT: usize = 2;
    const PAGE_WRITE_THROUGH_SHIFT: usize = 3;
    const PAGE_CACHE_DISABLE_SHIFT: usize = 4;
    const ACCESSED_SHIFT: usize = 5;
    const DIRTY_SHIFT: usize = 6;
    const PAGE_SIZE_1_GIB_SHIFT: usize = 7;
    const RESTART_SHIFT: usize = 11;
    const EXECUTE_DISABLE_SHIFT: usize = 63;
    const PAGE_DIRECTORY_TABLE_SHIFT_BEGIN: usize = 12;
    const PAGE_DIRECTORY_TABLE_SHIFT_END: usize = 52;

    const PRESENT_MASK: u64 = 1 << Self::PRESENT_SHIFT;
    const WRITABLE_MASK: u64 = 1 << Self::WRITABLE_SHIFT;
    const USER_MODE_ACCESS_MASK: u64 = 1 << Self::USER_MODE_ACCESS_SHIFT;
    const PAGE_WRITE_THROUGH_MASK: u64 = 1 << Self::PAGE_WRITE_THROUGH_SHIFT;
    const PAGE_CACHE_DISABLE_MASK: u64 = 1 << Self::PAGE_CACHE_DISABLE_SHIFT;
    const ACCESSED_MASK: u64 = 1 << Self::ACCESSED_SHIFT;
    const DIRTY_MASK: u64 = 1 << Self::DIRTY_SHIFT;
    const PAGE_SIZE_1_GIB_MASK: u64 = 1 << Self::PAGE_SIZE_1_GIB_SHIFT;
    const RESTART_MASK: u64 = 1 << Self::RESTART_SHIFT;
    const EXECUTE_DISABLE_MASK: u64 = 1 << Self::EXECUTE_DISABLE_SHIFT;
    const PAGE_DIRECTORY_TABLE_MASK: u64 = (1 << Self::PAGE_DIRECTORY_TABLE_SHIFT_END) - (1 << Self::PAGE_DIRECTORY_TABLE_SHIFT_BEGIN);

    fn read(page_map_level_4_entry: u64) -> Option<Self> {
        if page_map_level_4_entry & Self::PRESENT_MASK != 0 {
            let writable: bool = page_map_level_4_entry & Self::WRITABLE_MASK != 0;
            let user_mode_access: bool = page_map_level_4_entry & Self::USER_MODE_ACCESS_MASK != 0;
            let page_write_through: bool = page_map_level_4_entry & Self::PAGE_WRITE_THROUGH_MASK != 0;
            let page_cache_disable: bool = page_map_level_4_entry & Self::PAGE_CACHE_DISABLE_MASK != 0;
            let accessed: bool = page_map_level_4_entry & Self::ACCESSED_MASK != 0;
            let dirty: bool = page_map_level_4_entry & Self::DIRTY_MASK != 0;
            let page_size_1_gib: bool = page_map_level_4_entry & Self::PAGE_SIZE_1_GIB_MASK != 0;
            let restart: bool = page_map_level_4_entry & Self::RESTART_MASK != 0;
            let execute_disable: bool = page_map_level_4_entry & Self::EXECUTE_DISABLE_MASK != 0;
            let page_directory_table: u64 = page_map_level_4_entry & Self::PAGE_DIRECTORY_TABLE_MASK;
            let page_directory_table: *const [u64; ENTRIES] = page_directory_table as *const [u64; ENTRIES];
            let page_directory_table: &[u64; ENTRIES] = unsafe {
                &*page_directory_table
            };
            let page_directory_table: Option<&[u64; ENTRIES]> = if page_size_1_gib {
                None
            } else {
                Some(page_directory_table)
            };
            Some(Self {
                writable,
                user_mode_access,
                page_write_through,
                page_cache_disable,
                accessed,
                dirty,
                page_size_1_gib,
                restart,
                execute_disable,
                page_directory_table,
            })
        } else {
            None
        }
    }
}

