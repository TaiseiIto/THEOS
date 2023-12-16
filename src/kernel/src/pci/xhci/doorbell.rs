use core::fmt;

// https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf
// 5.6 Doorbell Registers
pub struct Registers(u32);

impl Registers {
    const DB_TARGET_BEGIN: usize = 0;
    const DB_TARGET_LENGTH: usize = 8;
    const DB_TARGET_END: usize = Self::DB_TARGET_BEGIN + Self::DB_TARGET_LENGTH;
    const DB_TASK_ID_BEGIN: usize = Self::DB_TARGET_END + 8;

    const DB_TARGET_MASK: u32 = (1 << Self::DB_TARGET_END) - (1 << Self::DB_TARGET_BEGIN);
    const DB_TASK_ID_MASK: u32 = u32::MAX - (1 << Self::DB_TASK_ID_BEGIN) + 1;

    fn db_target(&self) -> u8 {
        ((self.0 & Self::DB_TARGET_MASK) >> Self::DB_TARGET_BEGIN) as u8
    }

    fn db_task_id(&self) -> u16 {
        ((self.0 & Self::DB_TASK_ID_MASK) >> Self::DB_TASK_ID_BEGIN) as u16
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Doorbell")
            .field("self", &self.0)
            .field("DB_TARGET", &self.db_target())
            .field("DB_TASK_ID", &self.db_task_id())
            .finish()
    }
}

