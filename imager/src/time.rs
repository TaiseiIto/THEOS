use std::{
    ffi,
    os::raw,
    path,
};

#[repr(C)]
struct TimeSpec {
    tv_sec: u64,
    tv_nsec: u32,
}

#[link(name="time", kind="static")]
extern "C" {
    fn current_time() -> TimeSpec;
    fn last_accessed_time(path: *const raw::c_char) -> TimeSpec;
    fn last_changed_time(path: *const raw::c_char) -> TimeSpec;
    fn last_modified_time(path: *const raw::c_char) -> TimeSpec;
}

const FAT_YEAR: u64 = 1980;
const GREGORIAN_YEAR: u64 = 1582;
const GREGORIAN_MONTH: u8 = 10;
const GREGORIAN_DAY: u8 = 15;
const UNIX_YEAR: u64 = 1970;

#[derive(Clone, Copy, Debug)]
pub struct Time {
    year: u64,
    month: u8,
    day: u8,
    hour: u8,
    min: u8,
    sec: u8,
    nsec: u32,
}

impl Time {
    pub fn current_time() -> Self {
        Self::from_time_spec(unsafe {
            current_time()
        })
    }

    pub fn last_accessed_time(path: &path::PathBuf) -> Self {
        if !path.exists() {
            panic!("\"{}\" is not found.", path.display());
        }
        let path: &str = path.to_str().expect("Can't convert PathBuf to &str");
        let path = ffi::CString::new(path).expect("Can't create CString.");
        let path: *const raw::c_char = path.as_ptr();
        unsafe {
            Self::from_time_spec(last_accessed_time(path))
        }
    }

    pub fn last_changed_time(path: &path::PathBuf) -> Self {
        if !path.exists() {
            panic!("\"{}\" is not found.", path.display());
        }
        let path: &str = path.to_str().expect("Can't convert PathBuf to &str");
        let path = ffi::CString::new(path).expect("Can't create CString.");
        let path: *const raw::c_char = path.as_ptr();
        unsafe {
            Self::from_time_spec(last_changed_time(path))
        }
    }

    pub fn last_modified_time(path: &path::PathBuf) -> Self {
        if !path.exists() {
            panic!("\"{}\" is not found.", path.display());
        }
        let path: &str = path.to_str().expect("Can't convert PathBuf to &str");
        let path = ffi::CString::new(path).expect("Can't create CString.");
        let path: *const raw::c_char = path.as_ptr();
        unsafe {
            Self::from_time_spec(last_modified_time(path))
        }
    }

    pub fn fat_timestamp(&self) -> u32 {
        let double_seconds: u32 = (self.sec as u32) / 2;
        let minute: u32 = (self.min as u32) << 5;
        let hour: u32 = (self.hour as u32) << 11;
        let day: u32 = (self.day as u32) << 16;
        let month: u32 = (self.month as u32) << 21;
        let year: u32 = (self.year - FAT_YEAR << 25) as u32;
        year + month + day + hour + minute + double_seconds
    }

    pub fn get_10ms_increment(&self) -> u8 {
        let sec: u8 = 100 * (self.sec % 2);
        let msec: u8 = (self.nsec / 10000) as u8;
        sec + msec
    }

    pub fn guid_timestamp(&self) -> u64 {
        let days: u64 =
            (day_per_month(GREGORIAN_YEAR, GREGORIAN_MONTH) as u64)
            - (GREGORIAN_DAY as u64) + 1
            + (GREGORIAN_MONTH + 1..=12)
                .map(|month| day_per_month(GREGORIAN_YEAR, month) as u64)
                .sum::<u64>()
            + (GREGORIAN_YEAR + 1..self.year)
                .map(|year| (1..=12).map(move |month| (year, month)))
                .flatten()
                .map(|(year, month)| day_per_month(year, month) as u64)
                .sum::<u64>()
            + (1..self.month)
                .map(|month| day_per_month(self.year, month) as u64)
                .sum::<u64>()
            + (self.day as u64) - 1;
        let hours: u64 = 24 * days + (self.hour as u64);
        let minutes: u64 = 60 * hours + (self.min as u64);
        let seconds: u64 = 60 * minutes + (self.sec as u64);
        10000000 * seconds + (self.nsec as u64) / 100
    }

    pub fn new(year: u64, month: u8, day: u8, hour: u8, min: u8, sec: u8, nsec: u32) -> Self {
        if month < 1 || 12 < month {
            panic!("month < 1 || 12 < month");
        }
        if day < 1 || day_per_month(year, month) < day {
            panic!("day < 1 || day_per_month(year, month) < day");
        }
        if 24 <= hour {
            panic!("24 <= hour");
        }
        if 60 <= min {
            panic!("60 <= min");
        }
        if 60 <= sec {
            panic!("60 <= sec");
        }
        if 1000000000 <= nsec {
            panic!("1000000000 <= nsec");
        }
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

    pub fn unix_timestamp(&self) -> u64 {
        let days: u64 =
            (UNIX_YEAR..self.year)
                .map(|year| (1..=12).map(move |month| (year, month)))
                .flatten()
                .map(|(year, month)| day_per_month(year, month) as u64)
                .sum::<u64>()
            + (1..self.month)
                .map(|month| day_per_month(self.year, month) as u64)
                .sum::<u64>()
            + (self.day as u64) - 1;
        let hours: u64 = 24 * days + (self.hour as u64);
        let minutes: u64 = 60 * hours + (self.min as u64);
        let seconds: u64 = 60 * minutes + (self.sec as u64);
        seconds
    }

    pub fn utc_offset(&self) -> u8 {
        0
    }

    fn from_time_spec(time: TimeSpec) -> Self {
        let unix_sec = time.tv_sec;
        let nsec = time.tv_nsec;
        let sec_per_min = 60;
        let min_per_hour = 60;
        let hour_per_day = 24;
        let sec = unix_sec % sec_per_min;
        let unix_min = unix_sec / sec_per_min;
        let min = unix_min % min_per_hour;
        let unix_hour = unix_min / min_per_hour;
        let hour = unix_hour % hour_per_day;
        let unix_day = unix_hour / hour_per_day;
        let mut year = UNIX_YEAR;
        let mut month = 1;
        let mut day = unix_day + 1;
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
}

fn day_per_month(year: u64, month: u8) -> u8 {
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

fn is_leap_year(year: u64) -> bool {
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

