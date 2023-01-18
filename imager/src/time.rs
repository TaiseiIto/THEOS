use std::{
    ffi,
    os::raw,
    path,
};

#[repr(packed)]
struct TimeSpec {
    tv_sec: u64,
    tv_nsec: u32,
}

#[link(name="stat", kind="static")]
extern "C" {
    fn get_accessed_time_sec(path: *const raw::c_char) -> u64;
    fn get_accessed_time_nsec(path: *const raw::c_char) -> u32;
    fn get_changed_time_sec(path: *const raw::c_char) -> u64;
    fn get_changed_time_nsec(path: *const raw::c_char) -> u32;
    fn get_modified_time_sec(path: *const raw::c_char) -> u64;
    fn get_modified_time_nsec(path: *const raw::c_char) -> u32;
    fn get_current_time() -> TimeSpec;
}

const UNIX_YEAR: u32 = 1970;
const FAT_YEAR: u32 = 1980;

#[derive(Debug)]
pub struct Time {
    year: u32,
    month: u8,
    day: u8,
    hour: u8,
    min: u8,
    sec: u8,
    nsec: u32,
}

impl Time {
    pub fn new(unix_epoch_sec: u64, nsec: u32) -> Self {
        let sec_per_min = 60;
        let min_per_hour = 60;
        let hour_per_day = 24;
        let sec = unix_epoch_sec % sec_per_min;
        let unix_epoch_min = unix_epoch_sec / sec_per_min;
        let min = unix_epoch_min % min_per_hour;
        let unix_epoch_hour = unix_epoch_min / min_per_hour;
        let hour = unix_epoch_hour % hour_per_day;
        let unix_epoch_day = unix_epoch_hour / hour_per_day;
        let mut year = UNIX_YEAR;
        let mut month = 1;
        let mut day = unix_epoch_day + 1;
        while (day_per_month(year, month) as u64) < day {
            day -= day_per_month(year, month) as u64;
            if month < 12 {
                month += 1;
            } else if month == 12 {
                year += 1;
                month = 1;
            }
        }
        let month: u8 = month as u8;
        let day: u8 = day as u8;
        let hour: u8 = hour as u8;
        let min: u8 = min as u8;
        let sec: u8 = sec as u8;
        Self {
            year,
            month,
            day,
            hour,
            min,
            sec,
            nsec,
        }
    }

    pub fn get_current_time() -> Self {
        let current_time_spec: TimeSpec = unsafe {
            get_current_time()
        };
        Self::new(current_time_spec.tv_sec, current_time_spec.tv_nsec)
    }

    pub fn get_accessed_time(path: &path::PathBuf) -> Self {
        if !path.exists() {
            panic!("\"{}\" is not found.", path.display());
        }
        let path: &str = path.to_str().expect("Can't convert PathBuf to &str");
        let path = ffi::CString::new(path).expect("Can't create CString.");
        let path: *const raw::c_char = path.as_ptr();
        unsafe {
            Self::new(get_accessed_time_sec(path), get_accessed_time_nsec(path))
        }
    }

    pub fn get_changed_time(path: &path::PathBuf) -> Self {
        if !path.exists() {
            panic!("\"{}\" is not found.", path.display());
        }
        let path: &str = path.to_str().expect("Can't convert PathBuf to &str");
        let path = ffi::CString::new(path).expect("Can't create CString.");
        let path: *const raw::c_char = path.as_ptr();
        unsafe {
            Self::new(get_changed_time_sec(path), get_changed_time_nsec(path))
        }
    }

    pub fn get_modified_time(path: &path::PathBuf) -> Self {
        if !path.exists() {
            panic!("\"{}\" is not found.", path.display());
        }
        let path: &str = path.to_str().expect("Can't convert PathBuf to &str");
        let path = ffi::CString::new(path).expect("Can't create CString.");
        let path: *const raw::c_char = path.as_ptr();
        unsafe {
            Self::new(get_modified_time_sec(path), get_modified_time_nsec(path))
        }
    }

    pub fn get_file_timestamp(&self) -> u32 {
        let double_seconds: u32 = (self.sec as u32) / 2;
        let minute: u32 = (self.min as u32) << 5;
        let hour: u32 = (self.hour as u32) << 11;
        let day: u32 = (self.day as u32) << 16;
        let month: u32 = (self.month as u32) << 21;
        let year: u32 = self.year - FAT_YEAR << 25;
        year + month + day + hour + minute + double_seconds
    }

    pub fn get_10ms_increment(&self) -> u8 {
        let sec: u8 = 100 * (self.sec % 2);
        let msec: u8 = (self.nsec / 10000) as u8;
        sec + msec
    }

    pub fn get_utc_offset(&self) -> u8 {
        0
    }
}

fn day_per_month(year: u32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) {
            29
        } else {
            28
        },
        _ => panic!("month exceeds 12!"),
    }
}

fn is_leap_year(year: u32) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                true
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    }
}

