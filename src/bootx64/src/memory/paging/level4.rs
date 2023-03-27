// References
// Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 3 System Programming Guide, Chapter 4 Paging, Section 5 4-Level Paging And 5-Level Paging

use {
    alloc::vec::Vec,
    core::slice,
    super::super::Pages,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Cr3<'a> {
    pwt: bool,
    pcd: bool,
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
        let page_map_level_4_table: *mut [u64; ENTRIES] = page_map_level_4_table as *mut [u64; ENTRIES];
        let page_map_level_4_table: &mut [u64; ENTRIES] = unsafe {
            &mut *page_map_level_4_table
        };
        let page_map_level_4_entries: Vec<PageMapLevel4Entry> = page_map_level_4_table
            .iter_mut()
            .enumerate()
            .filter_map(|(i, page_map_level_4_entry)| PageMapLevel4Entry::read(i << 39, page_map_level_4_entry))
            .collect();
        Self {
            pwt,
            pcd,
            page_map_level_4_entries,
        }
	}
}

#[allow(dead_code)]
#[derive(Debug)]
struct PageMapLevel4Entry<'a> {
    virtual_address: usize,
    page_map_level_4_entry: &'a mut u64,
    writable: bool,
    user_mode_access: bool,
    page_write_through: bool,
    page_cache_disable: bool,
    accessed: bool,
    restart: bool,
    page_directory_pointer_entries: Vec<PageDirectoryPointerEntry<'a>>,
    execute_disable: bool,
}

impl<'a> PageMapLevel4Entry<'a> {
    const PRESENT_SHIFT: usize = 0;
    const WRITABLE_SHIFT: usize = 1;
    const USER_MODE_ACCESS_SHIFT: usize = 2;
    const PAGE_WRITE_THROUGH_SHIFT: usize = 3;
    const PAGE_CACHE_DISABLE_SHIFT: usize = 4;
    const ACCESSED_SHIFT: usize = 5;
    const RESTART_SHIFT: usize = 11;
    const PAGE_DIRECTORY_POINTER_TABLE_SHIFT_BEGIN: usize = 12;
    const PAGE_DIRECTORY_POINTER_TABLE_SHIFT_END: usize = 52;
    const EXECUTE_DISABLE_SHIFT: usize = 63;

    const PRESENT_MASK: u64 = 1 << Self::PRESENT_SHIFT;
    const WRITABLE_MASK: u64 = 1 << Self::WRITABLE_SHIFT;
    const USER_MODE_ACCESS_MASK: u64 = 1 << Self::USER_MODE_ACCESS_SHIFT;
    const PAGE_WRITE_THROUGH_MASK: u64 = 1 << Self::PAGE_WRITE_THROUGH_SHIFT;
    const PAGE_CACHE_DISABLE_MASK: u64 = 1 << Self::PAGE_CACHE_DISABLE_SHIFT;
    const ACCESSED_MASK: u64 = 1 << Self::ACCESSED_SHIFT;
    const RESTART_MASK: u64 = 1 << Self::RESTART_SHIFT;
    const PAGE_DIRECTORY_POINTER_TABLE_MASK: u64 = (1 << Self::PAGE_DIRECTORY_POINTER_TABLE_SHIFT_END) - (1 << Self::PAGE_DIRECTORY_POINTER_TABLE_SHIFT_BEGIN);
    const EXECUTE_DISABLE_MASK: u64 = 1 << Self::EXECUTE_DISABLE_SHIFT;

    fn read(virtual_address: usize, page_map_level_4_entry: &'a mut u64) -> Option<Self> {
        if *page_map_level_4_entry & Self::PRESENT_MASK != 0 {
            let writable: bool = *page_map_level_4_entry & Self::WRITABLE_MASK != 0;
            let user_mode_access: bool = *page_map_level_4_entry & Self::USER_MODE_ACCESS_MASK != 0;
            let page_write_through: bool = *page_map_level_4_entry & Self::PAGE_WRITE_THROUGH_MASK != 0;
            let page_cache_disable: bool = *page_map_level_4_entry & Self::PAGE_CACHE_DISABLE_MASK != 0;
            let accessed: bool = *page_map_level_4_entry & Self::ACCESSED_MASK != 0;
            let restart: bool = *page_map_level_4_entry & Self::RESTART_MASK != 0;
            let page_directory_pointer_table: u64 = *page_map_level_4_entry & Self::PAGE_DIRECTORY_POINTER_TABLE_MASK;
            let page_directory_pointer_table: *mut [u64; ENTRIES] = page_directory_pointer_table as *mut [u64; ENTRIES];
            let page_directory_pointer_table: &mut [u64; ENTRIES] = unsafe {
                &mut *page_directory_pointer_table
            };
            let page_directory_pointer_entries: Vec<PageDirectoryPointerEntry> = page_directory_pointer_table
                .iter_mut()
                .enumerate()
                .filter_map(|(i, page_directory_pointer_entry)| PageDirectoryPointerEntry::read(virtual_address + (i << 30), page_directory_pointer_entry))
                .collect();
            let execute_disable: bool = *page_map_level_4_entry & Self::EXECUTE_DISABLE_MASK != 0;
            Some(Self {
                virtual_address,
                page_map_level_4_entry,
                writable,
                user_mode_access,
                page_write_through,
                page_cache_disable,
                accessed,
                restart,
                page_directory_pointer_entries,
                execute_disable,
            })
        } else {
            None
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct PageDirectoryPointerEntry<'a> {
    virtual_address: usize,
    page_directory_pointer_entry: &'a mut u64,
    writable: bool,
    user_mode_access: bool,
    page_write_through: bool,
    page_cache_disable: bool,
    accessed: bool,
    dirty: bool,
    page_size_1_gib: bool,
    global: Option<bool>,
    restart: bool,
    page_attribute_table: Option<bool>,
    page_directory_table_page: Option<Pages<'a>>,
    page_directory_entries: Option<Vec<PageDirectoryEntry<'a>>>,
    page_1_gib_physical_address: Option<usize>,
    protection_key: Option<u8>,
    execute_disable: bool,
}

impl<'a> PageDirectoryPointerEntry<'a> {
    const PAGE_DIRECTORY_ENTRY_INDEX_SHIFT: usize = 21;

    const PRESENT_SHIFT: usize = 0;
    const WRITABLE_SHIFT: usize = 1;
    const USER_MODE_ACCESS_SHIFT: usize = 2;
    const PAGE_WRITE_THROUGH_SHIFT: usize = 3;
    const PAGE_CACHE_DISABLE_SHIFT: usize = 4;
    const ACCESSED_SHIFT: usize = 5;
    const DIRTY_SHIFT: usize = 6;
    const PAGE_SIZE_1_GIB_SHIFT: usize = 7;
    const GLOBAL_SHIFT: usize = 8;
    const RESTART_SHIFT: usize = 11;
    const PAGE_ATTRIBUTE_TABLE_SHIFT: usize = 12;
    const PAGE_DIRECTORY_TABLE_SHIFT_BEGIN: usize = 12;
    const PAGE_DIRECTORY_TABLE_SHIFT_END: usize = 52;
    const PAGE_1_GIB_SHIFT_BEGIN: usize = 30;
    const PAGE_1_GIB_SHIFT_END: usize = 52;
    const PROTECTION_KEY_SHIFT_BEGIN: usize = 59;
    const PROTECTION_KEY_SHIFT_END: usize = 63;
    const EXECUTE_DISABLE_SHIFT: usize = 63;

    const PRESENT_MASK: u64 = 1 << Self::PRESENT_SHIFT;
    const WRITABLE_MASK: u64 = 1 << Self::WRITABLE_SHIFT;
    const USER_MODE_ACCESS_MASK: u64 = 1 << Self::USER_MODE_ACCESS_SHIFT;
    const PAGE_WRITE_THROUGH_MASK: u64 = 1 << Self::PAGE_WRITE_THROUGH_SHIFT;
    const PAGE_CACHE_DISABLE_MASK: u64 = 1 << Self::PAGE_CACHE_DISABLE_SHIFT;
    const ACCESSED_MASK: u64 = 1 << Self::ACCESSED_SHIFT;
    const DIRTY_MASK: u64 = 1 << Self::DIRTY_SHIFT;
    const PAGE_SIZE_1_GIB_MASK: u64 = 1 << Self::PAGE_SIZE_1_GIB_SHIFT;
    const GLOBAL_MASK: u64 = 1 << Self::GLOBAL_SHIFT;
    const RESTART_MASK: u64 = 1 << Self::RESTART_SHIFT;
    const PAGE_ATTRIBUTE_TABLE_MASK: u64 = 1 << Self::PAGE_ATTRIBUTE_TABLE_SHIFT;
    const PAGE_DIRECTORY_TABLE_MASK: u64 = (1 << Self::PAGE_DIRECTORY_TABLE_SHIFT_END) - (1 << Self::PAGE_DIRECTORY_TABLE_SHIFT_BEGIN);
    const PAGE_1_GIB_MASK: u64 = (1 << Self::PAGE_1_GIB_SHIFT_END) - (1 << Self::PAGE_1_GIB_SHIFT_BEGIN);
    const PROTECTION_KEY_MASK: u64 = (1 << Self::PROTECTION_KEY_SHIFT_END) - (1 << Self::PROTECTION_KEY_SHIFT_BEGIN);
    const EXECUTE_DISABLE_MASK: u64 = 1 << Self::EXECUTE_DISABLE_SHIFT;

    fn read(virtual_address: usize, page_directory_pointer_entry: &'a mut u64) -> Option<Self> {
        if *page_directory_pointer_entry & Self::PRESENT_MASK != 0 {
            let writable: bool = *page_directory_pointer_entry & Self::WRITABLE_MASK != 0;
            let user_mode_access: bool = *page_directory_pointer_entry & Self::USER_MODE_ACCESS_MASK != 0;
            let page_write_through: bool = *page_directory_pointer_entry & Self::PAGE_WRITE_THROUGH_MASK != 0;
            let page_cache_disable: bool = *page_directory_pointer_entry & Self::PAGE_CACHE_DISABLE_MASK != 0;
            let accessed: bool = *page_directory_pointer_entry & Self::ACCESSED_MASK != 0;
            let dirty: bool = *page_directory_pointer_entry & Self::DIRTY_MASK != 0;
            let page_size_1_gib: bool = *page_directory_pointer_entry & Self::PAGE_SIZE_1_GIB_MASK != 0;
            let global: Option<bool> = if page_size_1_gib {
                Some(*page_directory_pointer_entry & Self::GLOBAL_MASK != 0)
            } else {
                None
            };
            let restart: bool = *page_directory_pointer_entry & Self::RESTART_MASK != 0;
            let page_attribute_table: Option<bool> = if page_size_1_gib {
                Some(*page_directory_pointer_entry & Self::PAGE_ATTRIBUTE_TABLE_MASK != 0)
            } else {
                None
            };
            let page_directory_table: u64 = *page_directory_pointer_entry & Self::PAGE_DIRECTORY_TABLE_MASK;
            let page_directory_table: *mut [u64; ENTRIES] = page_directory_table as *mut [u64; ENTRIES];
            let page_directory_table: &mut [u64; ENTRIES] = unsafe {
                &mut *page_directory_table
            };
            let page_directory_table_page: Option<Pages> = None;
            let page_directory_entries: Option<Vec<PageDirectoryEntry>> = if page_size_1_gib {
                None
            } else {
                Some(
                    page_directory_table
                        .iter_mut()
                        .enumerate()
                        .filter_map(|(i, page_directory_entry)| PageDirectoryEntry::read(virtual_address + (i << 21), page_directory_entry))
                        .collect()
                )
            };
            let page_1_gib_physical_address: Option<usize> = if page_size_1_gib {
                Some((*page_directory_pointer_entry & Self::PAGE_1_GIB_MASK) as usize)
            } else {
                None
            };
            let protection_key: Option<u8> = if page_size_1_gib {
                Some(((*page_directory_pointer_entry & Self::PROTECTION_KEY_MASK) >> Self::PROTECTION_KEY_SHIFT_BEGIN) as u8)
            } else {
                None
            };
            let execute_disable: bool = *page_directory_pointer_entry & Self::EXECUTE_DISABLE_MASK != 0;
            Some(Self {
                virtual_address,
                page_directory_pointer_entry,
                writable,
                user_mode_access,
                page_write_through,
                page_cache_disable,
                accessed,
                dirty,
                page_size_1_gib,
                global,
                restart,
                page_attribute_table,
                page_directory_table_page,
                page_directory_entries,
                page_1_gib_physical_address,
                protection_key,
                execute_disable,
            })
        } else {
            None
        }
    }

    fn divide(&mut self) {
        if !self.divided() {
            self.page_size_1_gib = false;
            self.global = None;
            self.page_attribute_table = None;
            self.page_directory_table_page = Some(Pages::new(1));
            let page_directory_table_page: &mut [u8] = self.page_directory_table_page
                .as_mut()
                .expect("Can't divide a page!")
                .bytes();
            let page_directory_table_page_len: usize = page_directory_table_page.len();
            let page_directory_table_page: *mut u8 = page_directory_table_page.as_mut_ptr();
            let page_directory_table_page: *mut u64 = page_directory_table_page as *mut u64;
            let page_directory_table_page_len: usize = page_directory_table_page_len / 8;
            let page_directory_table_page: &mut [u64] = unsafe {
                slice::from_raw_parts_mut(page_directory_table_page, page_directory_table_page_len)
            };
            self.page_directory_entries = Some(
                page_directory_table_page
                    .iter_mut()
                    .enumerate()
                    .map(|(i, page_directory_entry)| PageDirectoryEntry::new(
                        self.virtual_address + (i << Self::PAGE_DIRECTORY_ENTRY_INDEX_SHIFT),
                        page_directory_entry,
                        self.writable,
                        self.user_mode_access,
                        self.page_write_through,
                        self.page_cache_disable,
                        self.global.expect("Can't divide a page!"),
                        self.restart,
                        self.page_attribute_table.expect("Can't divide a page!"),
                        self.page_1_gib_physical_address.expect("Can't divide a page!") + (i << Self::PAGE_DIRECTORY_ENTRY_INDEX_SHIFT),
                        self.protection_key.expect("Can't divide a page!"),
                        self.execute_disable,
                    ))
                    .collect()
            );
            self.page_1_gib_physical_address = None;
            self.protection_key = None;
            let present: u64 = Self::PRESENT_MASK;
            let writable: u64 = if self.writable {
                Self::WRITABLE_MASK
            } else {
                0
            };
            let user_mode_access: u64 = if self.user_mode_access {
                Self::USER_MODE_ACCESS_MASK
            } else {
                0
            };
            let page_write_through: u64 = if self.page_write_through {
                Self::PAGE_WRITE_THROUGH_MASK
            } else {
                0
            };
            let page_cache_disable: u64 = if self.page_cache_disable {
                Self::PAGE_CACHE_DISABLE_MASK
            } else {
                0
            };
            let accessed: u64 = if self.accessed {
                Self::ACCESSED_MASK
            } else {
                0
            };
            let restart: u64 = if self.restart {
                Self::RESTART_MASK
            } else {
                0
            };
            let page_directory_table: u64 = self.page_directory_table_page
                .as_ref()
                .expect("Can't divide a page!")
                .physical_address();
            let execute_disable: u64 = if self.execute_disable {
                Self::EXECUTE_DISABLE_MASK
            } else {
                0
            };
            *self.page_directory_pointer_entry =
                present
                | writable
                | user_mode_access
                | page_write_through
                | page_cache_disable
                | accessed
                | restart
                | page_directory_table
                | execute_disable;
        }
    }

    fn divided(&self) -> bool {
        self.page_directory_entries.is_some()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct PageDirectoryEntry<'a> {
    virtual_address: usize,
    page_directory_entry: &'a mut u64,
    writable: bool,
    user_mode_access: bool,
    page_write_through: bool,
    page_cache_disable: bool,
    accessed: bool,
    dirty: bool,
    page_size_2_mib: bool,
    global: Option<bool>,
    restart: bool,
    page_attribute_table: Option<bool>,
    page_table_page: Option<Pages<'a>>,
    page_entries: Option<Vec<PageEntry<'a>>>,
    page_2_mib_physical_address: Option<usize>,
    protection_key: Option<u8>,
    execute_disable: bool,
}

impl<'a> PageDirectoryEntry<'a> {
    const PAGE_ENTRY_INDEX_SHIFT: usize = 12;

    const PRESENT_SHIFT: usize = 0;
    const WRITABLE_SHIFT: usize = 1;
    const USER_MODE_ACCESS_SHIFT: usize = 2;
    const PAGE_WRITE_THROUGH_SHIFT: usize = 3;
    const PAGE_CACHE_DISABLE_SHIFT: usize = 4;
    const ACCESSED_SHIFT: usize = 5;
    const DIRTY_SHIFT: usize = 6;
    const PAGE_SIZE_2_MIB_SHIFT: usize = 7;
    const GLOBAL_SHIFT: usize = 8;
    const RESTART_SHIFT: usize = 11;
    const PAGE_ATTRIBUTE_TABLE_SHIFT: usize = 12;
    const PAGE_DIRECTORY_TABLE_SHIFT_BEGIN: usize = 12;
    const PAGE_DIRECTORY_TABLE_SHIFT_END: usize = 52;
    const PAGE_2_MIB_SHIFT_BEGIN: usize = 21;
    const PAGE_2_MIB_SHIFT_END: usize = 52;
    const PROTECTION_KEY_SHIFT_BEGIN: usize = 59;
    const PROTECTION_KEY_SHIFT_END: usize = 63;
    const EXECUTE_DISABLE_SHIFT: usize = 63;

    const PRESENT_MASK: u64 = 1 << Self::PRESENT_SHIFT;
    const WRITABLE_MASK: u64 = 1 << Self::WRITABLE_SHIFT;
    const USER_MODE_ACCESS_MASK: u64 = 1 << Self::USER_MODE_ACCESS_SHIFT;
    const PAGE_WRITE_THROUGH_MASK: u64 = 1 << Self::PAGE_WRITE_THROUGH_SHIFT;
    const PAGE_CACHE_DISABLE_MASK: u64 = 1 << Self::PAGE_CACHE_DISABLE_SHIFT;
    const ACCESSED_MASK: u64 = 1 << Self::ACCESSED_SHIFT;
    const DIRTY_MASK: u64 = 1 << Self::DIRTY_SHIFT;
    const PAGE_SIZE_2_MIB_MASK: u64 = 1 << Self::PAGE_SIZE_2_MIB_SHIFT;
    const GLOBAL_MASK: u64 = 1 << Self::GLOBAL_SHIFT;
    const RESTART_MASK: u64 = 1 << Self::RESTART_SHIFT;
    const PAGE_ATTRIBUTE_TABLE_MASK: u64 = 1 << Self::PAGE_ATTRIBUTE_TABLE_SHIFT;
    const PAGE_DIRECTORY_TABLE_MASK: u64 = (1 << Self::PAGE_DIRECTORY_TABLE_SHIFT_END) - (1 << Self::PAGE_DIRECTORY_TABLE_SHIFT_BEGIN);
    const PAGE_2_MIB_MASK: u64 = (1 << Self::PAGE_2_MIB_SHIFT_END) - (1 << Self::PAGE_2_MIB_SHIFT_BEGIN);
    const PROTECTION_KEY_MASK: u64 = (1 << Self::PROTECTION_KEY_SHIFT_END) - (1 << Self::PROTECTION_KEY_SHIFT_BEGIN);
    const EXECUTE_DISABLE_MASK: u64 = 1 << Self::EXECUTE_DISABLE_SHIFT;

    fn new(
        virtual_address: usize,
        page_directory_entry: &'a mut u64,
        writable: bool,
        user_mode_access: bool,
        page_write_through: bool,
        page_cache_disable: bool,
        global: bool,
        restart: bool,
        page_attribute_table: bool,
        physical_address: usize,
        protection_key: u8,
        execute_disable: bool,
    ) -> Self {
        let present_bit: u64 = Self::PRESENT_MASK;
        let writable_bit: u64 = if writable {
            Self::WRITABLE_MASK
        } else {
            0
        };
        let user_mode_access_bit: u64 = if user_mode_access {
            Self::USER_MODE_ACCESS_MASK
        } else {
            0
        };
        let page_write_through_bit: u64 = if page_write_through {
            Self::PAGE_WRITE_THROUGH_MASK
        } else {
            0
        };
        let page_cache_disable_bit: u64 = if page_cache_disable {
            Self::PAGE_CACHE_DISABLE_MASK
        } else {
            0
        };
        let accessed: bool = false;
        let accessed_bit: u64 = if accessed {
            Self::ACCESSED_MASK
        } else {
            0
        };
        let dirty: bool = false;
        let dirty_bit: u64 = if dirty {
            Self::DIRTY_MASK
        } else {
            0
        };
        let page_size_2_mib: bool = true;
        let page_size_2_mib_bit: u64 = if page_size_2_mib {
            Self::PAGE_2_MIB_MASK
        } else {
            0
        };
        let global: Option<bool> = Some(global);
        let global_bit: u64 = match global {
            Some(true) => Self::GLOBAL_MASK,
            _ => 0,
        };
        let restart_bit: u64 = if restart {
            Self::RESTART_MASK
        } else {
            0
        };
        let page_attribute_table: Option<bool> = Some(page_attribute_table);
        let page_attribute_table_bit: u64 = match page_attribute_table {
            Some(true) => Self::PAGE_ATTRIBUTE_TABLE_MASK,
            _ => 0,
        };
        let page_table_page: Option<Pages> = None;
        let page_entries: Option<Vec<PageEntry>> = None;
        let page_2_mib_physical_address: Option<usize> = Some(physical_address);
        let page_2_mib_physical_address_bits: u64 = physical_address as u64 & Self::PAGE_2_MIB_MASK;
        let protection_key: Option<u8> = Some(protection_key);
        let protection_key_bits: u64 = match protection_key {
            Some(protection_key) => (protection_key as u64) << Self::PROTECTION_KEY_SHIFT_BEGIN,
            None => 0,
        };
        let execute_disable_bit: u64 = if execute_disable {
            Self::EXECUTE_DISABLE_MASK
        } else {
            0
        };
        *page_directory_entry =
            present_bit
            | writable_bit
            | user_mode_access_bit
            | page_write_through_bit
            | page_cache_disable_bit
            | accessed_bit
            | dirty_bit
            | page_size_2_mib_bit
            | global_bit
            | restart_bit
            | page_attribute_table_bit
            | page_2_mib_physical_address_bits
            | protection_key_bits
            | execute_disable_bit;
        Self {
            virtual_address,
            page_directory_entry,
            writable,
            user_mode_access,
            page_write_through,
            page_cache_disable,
            accessed,
            dirty,
            page_size_2_mib,
            global,
            restart,
            page_attribute_table,
            page_table_page,
            page_entries,
            page_2_mib_physical_address,
            protection_key,
            execute_disable,
        }
    }

    fn read(virtual_address: usize, page_directory_entry: &'a mut u64) -> Option<Self> {
        if *page_directory_entry & Self::PRESENT_MASK != 0 {
            let writable: bool = *page_directory_entry & Self::WRITABLE_MASK != 0;
            let user_mode_access: bool = *page_directory_entry & Self::USER_MODE_ACCESS_MASK != 0;
            let page_write_through: bool = *page_directory_entry & Self::PAGE_WRITE_THROUGH_MASK != 0;
            let page_cache_disable: bool = *page_directory_entry & Self::PAGE_CACHE_DISABLE_MASK != 0;
            let accessed: bool = *page_directory_entry & Self::ACCESSED_MASK != 0;
            let dirty: bool = *page_directory_entry & Self::DIRTY_MASK != 0;
            let page_size_2_mib: bool = *page_directory_entry & Self::PAGE_SIZE_2_MIB_MASK != 0;
            let global: Option<bool> = if page_size_2_mib {
                Some(*page_directory_entry & Self::GLOBAL_MASK != 0)
            } else {
                None
            };
            let restart: bool = *page_directory_entry & Self::RESTART_MASK != 0;
            let page_attribute_table: Option<bool> = if page_size_2_mib {
                Some(*page_directory_entry & Self::PAGE_ATTRIBUTE_TABLE_MASK != 0)
            } else {
                None
            };
            let page_table: u64 = *page_directory_entry & Self::PAGE_DIRECTORY_TABLE_MASK;
            let page_table: *mut [u64; ENTRIES] = page_table as *mut [u64; ENTRIES];
            let page_table: &mut [u64; ENTRIES] = unsafe {
                &mut *page_table
            };
            let page_table_page: Option<Pages> = None;
            let page_entries: Option<Vec<PageEntry>> = if page_size_2_mib {
                None
            } else {
                Some(
                    page_table
                        .iter_mut()
                        .enumerate()
                        .filter_map(|(i, page_entry)| PageEntry::read(virtual_address + (i << Self::PAGE_ENTRY_INDEX_SHIFT), page_entry))
                        .collect()
                )
            };
            let page_2_mib_physical_address: Option<usize> = if page_size_2_mib {
                Some((*page_directory_entry& Self::PAGE_2_MIB_MASK) as usize)
            } else {
                None
            };
            let protection_key: Option<u8> = if page_size_2_mib {
                Some(((*page_directory_entry & Self::PROTECTION_KEY_MASK) >> Self::PROTECTION_KEY_SHIFT_BEGIN) as u8)
            } else {
                None
            };
            let execute_disable: bool = *page_directory_entry & Self::EXECUTE_DISABLE_MASK != 0;
            Some(Self {
                virtual_address,
                page_directory_entry,
                writable,
                user_mode_access,
                page_write_through,
                page_cache_disable,
                accessed,
                dirty,
                page_size_2_mib,
                global,
                restart,
                page_attribute_table,
                page_table_page,
                page_entries,
                page_2_mib_physical_address,
                protection_key,
                execute_disable,
            })
        } else {
            None
        }
    }

    fn divide(&mut self) {
        if !self.divided() {
            self.page_size_2_mib = false;
            self.global = None;
            self.page_attribute_table = None;
            self.page_table_page = Some(Pages::new(1));
            let page_table_page: &mut [u8] = self.page_table_page
                .as_mut()
                .expect("Can't divide a page!")
                .bytes();
            let page_table_page_len: usize = page_table_page.len();
            let page_table_page: *mut u8 = page_table_page.as_mut_ptr();
            let page_table_page: *mut u64 = page_table_page as *mut u64;
            let page_table_page_len: usize = page_table_page_len / 8;
            let page_table_page: &mut [u64] = unsafe {
                slice::from_raw_parts_mut(page_table_page, page_table_page_len)
            };
            self.page_entries = Some(
                page_table_page
                    .iter_mut()
                    .enumerate()
                    .map(|(i, page_entry)| PageEntry::new(
                        self.virtual_address + (i << Self::PAGE_ENTRY_INDEX_SHIFT),
                        page_entry,
                        self.writable,
                        self.user_mode_access,
                        self.page_write_through,
                        self.page_cache_disable,
                        self.page_attribute_table.expect("Can't divide a page!"),
                        self.global.expect("Can't divide a page!"),
                        self.restart,
                        self.page_2_mib_physical_address.expect("Can't divide a page!") + (i << Self::PAGE_ENTRY_INDEX_SHIFT),
                        self.protection_key.expect("Can't divide a page!"),
                        self.execute_disable,
                    ))
                    .collect()
            );
            self.page_2_mib_physical_address = None;
            self.protection_key = None;
            let present: u64 = Self::PRESENT_MASK;
            let writable: u64 = if self.writable {
                Self::WRITABLE_MASK
            } else {
                0
            };
            let user_mode_access: u64 = if self.user_mode_access {
                Self::USER_MODE_ACCESS_MASK
            } else {
                0
            };
            let page_write_through: u64 = if self.page_write_through {
                Self::PAGE_WRITE_THROUGH_MASK
            } else {
                0
            };
            let page_cache_disable: u64 = if self.page_cache_disable {
                Self::PAGE_CACHE_DISABLE_MASK
            } else {
                0
            };
            let accessed: u64 = if self.accessed {
                Self::ACCESSED_MASK
            } else {
                0
            };
            let restart: u64 = if self.restart {
                Self::RESTART_MASK
            } else {
                0
            };
            let page_table: u64 = self.page_table_page
                .as_ref()
                .expect("Can't divide a page!")
                .physical_address();
            let execute_disable: u64 = if self.execute_disable {
                Self::EXECUTE_DISABLE_MASK
            } else {
                0
            };
            *self.page_directory_entry =
                present
                | writable
                | user_mode_access
                | page_write_through
                | page_cache_disable
                | accessed
                | restart
                | page_table
                | execute_disable;
        }
    }

    fn divided(&self) -> bool {
        self.page_entries.is_some()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct PageEntry<'a> {
    virtual_address: usize,
    page_entry: &'a mut u64,
    writable: bool,
    user_mode_access: bool,
    page_write_through: bool,
    page_cache_disable: bool,
    accessed: bool,
    dirty: bool,
    page_attribute_table: bool,
    global: bool,
    restart: bool,
    physical_address: usize,
    protection_key: u8,
    execute_disable: bool,
}

impl<'a> PageEntry<'a> {
    const PRESENT_SHIFT: usize = 0;
    const WRITABLE_SHIFT: usize = 1;
    const USER_MODE_ACCESS_SHIFT: usize = 2;
    const PAGE_WRITE_THROUGH_SHIFT: usize = 3;
    const PAGE_CACHE_DISABLE_SHIFT: usize = 4;
    const ACCESSED_SHIFT: usize = 5;
    const DIRTY_SHIFT: usize = 6;
    const PAGE_ATTRIBUTE_TABLE_SHIFT: usize = 7;
    const GLOBAL_SHIFT: usize = 8;
    const RESTART_SHIFT: usize = 11;
    const PHYSICAL_ADDRESS_SHIFT_BEGIN: usize = 12;
    const PHYSICAL_ADDRESS_SHIFT_END: usize = 52;
    const PROTECTION_KEY_SHIFT_BEGIN: usize = 59;
    const PROTECTION_KEY_SHIFT_END: usize = 63;
    const EXECUTE_DISABLE_SHIFT: usize = 63;

    const PRESENT_MASK: u64 = 1 << Self::PRESENT_SHIFT;
    const WRITABLE_MASK: u64 = 1 << Self::WRITABLE_SHIFT;
    const USER_MODE_ACCESS_MASK: u64 = 1 << Self::USER_MODE_ACCESS_SHIFT;
    const PAGE_WRITE_THROUGH_MASK: u64 = 1 << Self::PAGE_WRITE_THROUGH_SHIFT;
    const PAGE_CACHE_DISABLE_MASK: u64 = 1 << Self::PAGE_CACHE_DISABLE_SHIFT;
    const ACCESSED_MASK: u64 = 1 << Self::ACCESSED_SHIFT;
    const DIRTY_MASK: u64 = 1 << Self::DIRTY_SHIFT;
    const PAGE_ATTRIBUTE_TABLE_MASK: u64 = 1 << Self::PAGE_ATTRIBUTE_TABLE_SHIFT;
    const GLOBAL_MASK: u64 = 1 << Self::GLOBAL_SHIFT;
    const RESTART_MASK: u64 = 1 << Self::RESTART_SHIFT;
    const PHYSICAL_ADDRESS_MASK: u64 = (1 << Self::PHYSICAL_ADDRESS_SHIFT_END) - (1 << Self::PHYSICAL_ADDRESS_SHIFT_BEGIN);
    const PROTECTION_KEY_MASK: u64 = (1 << Self::PROTECTION_KEY_SHIFT_END) - (1 << Self::PROTECTION_KEY_SHIFT_BEGIN);
    const EXECUTE_DISABLE_MASK: u64 = 1 << Self::EXECUTE_DISABLE_SHIFT;

    fn new(
        virtual_address: usize,
        page_entry: &'a mut u64,
        writable: bool,
        user_mode_access: bool,
        page_write_through: bool,
        page_cache_disable: bool,
        page_attribute_table: bool,
        global: bool,
        restart: bool,
        physical_address: usize,
        protection_key: u8,
        execute_disable: bool,
    ) -> Self {
        let present_bit: u64 = Self::PRESENT_MASK;
        let writable_bit: u64 = if writable {
            Self::WRITABLE_MASK
        } else {
            0
        };
        let user_mode_access_bit: u64 = if user_mode_access {
            Self::USER_MODE_ACCESS_MASK
        } else {
            0
        };
        let page_write_through_bit: u64 = if page_write_through {
            Self::PAGE_WRITE_THROUGH_MASK
        } else {
            0
        };
        let page_cache_disable_bit: u64 = if page_cache_disable {
            Self::PAGE_CACHE_DISABLE_MASK
        } else {
            0
        };
        let accessed: bool = false;
        let accessed_bit: u64 = if accessed {
            Self::ACCESSED_MASK
        } else {
            0
        };
        let dirty: bool = false;
        let dirty_bit: u64 = if dirty {
            Self::DIRTY_MASK
        } else {
            0
        };
        let page_attribute_table_bit: u64 = if page_attribute_table {
            Self::PAGE_ATTRIBUTE_TABLE_MASK
        } else {
            0
        };
        let global_bit: u64 = if global {
            Self::PAGE_ATTRIBUTE_TABLE_MASK
        } else {
            0
        };
        let restart_bit: u64 = if restart {
            Self::RESTART_MASK
        } else {
            0
        };
        let physical_address_bits: u64 = physical_address as u64 & Self::PHYSICAL_ADDRESS_MASK;
        let physical_address: usize = physical_address as usize;
        let protection_key_bits: u64 = (protection_key as u64) << Self::PROTECTION_KEY_SHIFT_BEGIN;
        let execute_disable_bit: u64 = if execute_disable {
            Self::EXECUTE_DISABLE_MASK
        } else {
            0
        };
        *page_entry =
            present_bit
            | writable_bit
            | user_mode_access_bit
            | page_write_through_bit
            | page_cache_disable_bit
            | accessed_bit
            | dirty_bit
            | page_attribute_table_bit
            | global_bit
            | restart_bit
            | physical_address_bits
            | protection_key_bits
            | execute_disable_bit;
        Self {
            virtual_address,
            page_entry,
            writable,
            user_mode_access,
            page_write_through,
            page_cache_disable,
            accessed,
            dirty,
            page_attribute_table,
            global,
            restart,
            physical_address,
            protection_key,
            execute_disable,
        }
    }

    fn read(virtual_address: usize, page_entry: &'a mut u64) -> Option<Self> {
        if *page_entry & Self::PRESENT_MASK != 0 {
            let writable: bool = *page_entry & Self::WRITABLE_MASK != 0;
            let user_mode_access: bool = *page_entry & Self::USER_MODE_ACCESS_MASK != 0;
            let page_write_through: bool = *page_entry & Self::PAGE_WRITE_THROUGH_MASK != 0;
            let page_cache_disable: bool = *page_entry & Self::PAGE_CACHE_DISABLE_MASK != 0;
            let accessed: bool = *page_entry & Self::ACCESSED_MASK != 0;
            let dirty: bool = *page_entry & Self::DIRTY_MASK != 0;
            let page_attribute_table: bool = *page_entry & Self::PAGE_ATTRIBUTE_TABLE_MASK != 0;
            let global: bool = *page_entry & Self::GLOBAL_MASK != 0;
            let restart: bool = *page_entry & Self::RESTART_MASK != 0;
            let physical_address: usize = (*page_entry & Self::PHYSICAL_ADDRESS_MASK) as usize;
            let protection_key: u8 = ((*page_entry & Self::PROTECTION_KEY_MASK) >> Self::PROTECTION_KEY_SHIFT_BEGIN) as u8;
            let execute_disable: bool = *page_entry & Self::EXECUTE_DISABLE_MASK != 0;
            Some(Self {
                virtual_address,
                page_entry,
                writable,
                user_mode_access,
                page_write_through,
                page_cache_disable,
                accessed,
                dirty,
                page_attribute_table,
                global,
                restart,
                physical_address,
                protection_key,
                execute_disable,
            })
        } else {
            None
        }
    }

    fn set_physical_address(&mut self, physical_address: usize) {
        *self.page_entry &= !Self::PHYSICAL_ADDRESS_MASK;
        *self.page_entry |= physical_address as u64 & Self::PHYSICAL_ADDRESS_MASK;
        self.physical_address = physical_address;
    }
}

