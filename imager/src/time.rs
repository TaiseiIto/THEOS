use std::{
    fmt,
    fs,
    path::PathBuf,
    time,
};

const FAT_YEAR: i128 = 1980;
const GREGORIAN_DAY: u8 = 15;
const GREGORIAN_MONTH: u8 = 10;
const GREGORIAN_YEAR: i128 = 1582;
const FIRST_MONTH: u8 = 1;
const LAST_MONTH: u8 = 12;
const HOURS_PER_DAY: u8 = 24;
const MINUTES_PER_HOUR: u8 = 60;
const SECONDS_PER_MINUTE: u8 = 60;
const UNIX_YEAR: i128 = 1970;

#[derive(Clone, Copy, Debug)]
pub struct Time {
    year: i128,
    month: u8,
    day: u8,
    hour: u8,
    min: u8,
    sec: u8,
    nsec: u32,
}

impl Time {
    pub fn current_time() -> Self {
        Self::from_system_time(time::SystemTime::now())
    }

    pub fn fat_centi_second(&self) -> u8 {
        (self.sec % 2) * 100 + ((self.nsec / 10000000) as u8)
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

    pub fn from_fat_timestamp(timestamp: u32, t_10ms_increment: u8, utc_offset: i8) -> Self {
        let nsec: u32 = ((t_10ms_increment as u32) % 100) * 10000000;
        let sec: u8 = ((timestamp as u8) & 0x1f) * 2 + t_10ms_increment / 100;
        let min: u8 = ((timestamp >> 5) as u8) & 0x3f;
        let hour: u8 = ((timestamp >> 11) as u8) & 0x1f;
        let day: u8 = ((timestamp >> 16) as u8) & 0x1f;
        let month: u8 = ((timestamp >> 21) as u8) & 0x0f;
        let year: i128 = ((timestamp >> 25) as i128) + FAT_YEAR;
        let utc_offset_min: i128 = 15 * (utc_offset as i128);
        let utc_offset_sec: i128 = 60 * utc_offset_min;
        Self {
            year,
            month,
            day,
            hour,
            min,
            sec,
            nsec,
        }.add_sec(utc_offset_sec)
    }

    pub fn from_guid_timestamp(timestamp: u64) -> Self {
        let (nsec, sec): (u32, u64) = ((timestamp % 10000000 * 100) as u32, timestamp / 10000000);
        let (sec, min): (u8, u64) = ((sec % (SECONDS_PER_MINUTE as u64)) as u8, sec / (SECONDS_PER_MINUTE as u64));
        let (min, hour): (u8, u64) = ((min % (MINUTES_PER_HOUR as u64)) as u8, min / (MINUTES_PER_HOUR as u64));
        let (hour, day): (u8, u64) = ((hour % (HOURS_PER_DAY as u64)) as u8, hour / (HOURS_PER_DAY as u64));
        let mut day: u64 = day + (GREGORIAN_DAY as u64);
        let mut year: i128 = GREGORIAN_YEAR;
        let mut month: u8 = GREGORIAN_MONTH;
        while (month_length(year, month) as u64) < day {
            day -= month_length(year, month) as u64;
            (year, month) = next_month(year, month);
        }
        let day: u8 = day as u8;
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

    pub fn from_unix_timestamp(timestamp: i128) -> Self {
        let unix_epoch_offset: i128 = (0..UNIX_YEAR)
            .map(|year| (FIRST_MONTH..=LAST_MONTH).map(move |month| (year, month)))
            .flatten()
            .map(|(year, month)| month_length(year, month) as i128)
            .sum::<i128>()
            * HOURS_PER_DAY as i128
            * MINUTES_PER_HOUR as i128
            * SECONDS_PER_MINUTE as i128;
        let timestamp: i128 = unix_epoch_offset + timestamp;
        Self::from_sec(timestamp)
    }

    pub fn get_10ms_increment(&self) -> u8 {
        let sec: u8 = 100 * (self.sec % 2);
        let msec: u8 = (self.nsec / 10000000) as u8;
        sec + msec
    }

    pub fn guid_timestamp(&self) -> u64 {
        let days: u64 = if GREGORIAN_YEAR < self.year {
                (month_length(GREGORIAN_YEAR, GREGORIAN_MONTH) as u64)
                - (GREGORIAN_DAY as u64) + 1
                + (GREGORIAN_MONTH + 1..=LAST_MONTH)
                    .map(|month| month_length(GREGORIAN_YEAR, month) as u64)
                    .sum::<u64>()
                + (GREGORIAN_YEAR + 1..self.year)
                    .map(|year| (FIRST_MONTH..=LAST_MONTH).map(move |month| (year, month)))
                    .flatten()
                    .map(|(year, month)| month_length(year, month) as u64)
                    .sum::<u64>()
                + (1..self.month)
                    .map(|month| month_length(self.year, month) as u64)
                    .sum::<u64>()
                + (self.day as u64) - 1
            } else if self.year == GREGORIAN_YEAR {
                if GREGORIAN_MONTH < self.month {
                    (month_length(GREGORIAN_YEAR, GREGORIAN_MONTH) as u64)
                    - (GREGORIAN_DAY as u64) + 1
                    + (GREGORIAN_MONTH + 1..=self.month)
                        .map(|month| month_length(GREGORIAN_YEAR, month) as u64)
                        .sum::<u64>()
                    + (self.day as u64) - 1
                } else if self.month == GREGORIAN_MONTH {
                    if GREGORIAN_DAY <= self.day {
                        (self.day - GREGORIAN_DAY) as u64
                    } else {
                        panic!("Can't generate GUID timestamp.")
                    }
                } else {
                    panic!("Can't generate GUID timestamp.")
                }
            } else {
                panic!("Can't generate GUID timestamp.")
            };
        let hours: u64 = (HOURS_PER_DAY as u64) * days + (self.hour as u64);
        let minutes: u64 = (MINUTES_PER_HOUR as u64) * hours + (self.min as u64);
        let seconds: u64 = (SECONDS_PER_MINUTE as u64) * minutes + (self.sec as u64);
        10000000 * seconds + (self.nsec as u64) / 100
    }

    pub fn last_accessed_time(path: &PathBuf) -> Self {
        let metadata: fs::Metadata = fs::metadata(path).expect("Can't get an accessed time");
        let accessed_time: time::SystemTime = metadata
            .accessed()
            .expect("Can't get an accessed time!");
        Self::from_system_time(accessed_time)
    }

    pub fn last_changed_time(path: &PathBuf) -> Self {
        let metadata: fs::Metadata = fs::metadata(path).expect("Can't get an changed time");
        let changed_time: time::SystemTime = metadata
            .created()
            .unwrap_or(metadata
                .modified()
                .expect("Can't get a changed time!")
            );
        Self::from_system_time(changed_time)
    }

    pub fn last_modified_time(path: &PathBuf) -> Self {
        let metadata: fs::Metadata = fs::metadata(path).expect("Can't get an accessed time");
        let modified_time: time::SystemTime = metadata
            .modified()
            .expect("Can't get a modified time!");
        Self::from_system_time(modified_time)
    }

    pub fn new(year: i128, month: u8, day: u8, hour: u8, min: u8, sec: u8, nsec: u32) -> Self {
        assert!(FIRST_MONTH <= month && month <= LAST_MONTH);
        assert!(1 <= day && day <= month_length(year, month));
        assert!(hour < HOURS_PER_DAY);
        assert!(min < MINUTES_PER_HOUR);
        assert!(sec < SECONDS_PER_MINUTE);
        assert!(nsec < 1000000000);
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
                .map(|year| (FIRST_MONTH..=LAST_MONTH).map(move |month| (year, month)))
                .flatten()
                .map(|(year, month)| month_length(year, month) as u64)
                .sum::<u64>()
            + (1..self.month)
                .map(|month| month_length(self.year, month) as u64)
                .sum::<u64>()
            + (self.day as u64) - 1;
        let hours: u64 = (HOURS_PER_DAY as u64) * days + (self.hour as u64);
        let minutes: u64 = (MINUTES_PER_HOUR as u64) * hours + (self.min as u64);
        let seconds: u64 = (SECONDS_PER_MINUTE as u64) * minutes + (self.sec as u64);
        seconds
    }

    pub fn utc_offset(&self) -> i8 {
        0
    }

    fn add_sec(self, sec: i128) -> Self {
        Self::from_sec(self.to_sec() + sec)
    }

    fn from_system_time(time: time::SystemTime) -> Self {
        let nsec: u128 = time
            .duration_since(time::UNIX_EPOCH)
            .expect("Can't get time!")
            .as_nanos();
        Self::from_unix_nsec(nsec)
    }

    fn from_unix_nsec(nsec: u128) -> Self {
        let sec = (nsec / 1000000000) as u64;
        let nsec = (nsec % 1000000000) as u32;
        let (sec, min): (u8, u64) = ((sec % (SECONDS_PER_MINUTE as u64)) as u8, sec / (SECONDS_PER_MINUTE as u64));
        let (min, hour): (u8, u64) = ((min % (MINUTES_PER_HOUR as u64)) as u8, min / (MINUTES_PER_HOUR as u64));
        let (hour, day): (u8, u64) = ((hour % (HOURS_PER_DAY as u64)) as u8, hour / (HOURS_PER_DAY as u64));
        let mut year = UNIX_YEAR;
        let mut month = 1;
        let mut day = day + 1;
        while (month_length(year, month) as u64) < day {
            day -= month_length(year, month) as u64;
            (year, month) = next_month(year, month);
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

    fn from_sec(sec: i128) -> Self {
        let nsec: u32 = 0;
        let (sec, min): (u8, i128) = (sec.rem_euclid(SECONDS_PER_MINUTE as i128) as u8, sec.div_euclid(SECONDS_PER_MINUTE as i128));
        let (min, hour): (u8, i128) = (min.rem_euclid(MINUTES_PER_HOUR as i128) as u8, min.div_euclid(MINUTES_PER_HOUR as i128));
        let (hour, day): (u8, i128) = (hour.rem_euclid(HOURS_PER_DAY as i128) as u8, hour.div_euclid(HOURS_PER_DAY as i128));
        let mut year: i128;
        let mut month: u8;
        let mut day: i128 = day + 1;
        if 0 < day {
            year = 0;
            month = FIRST_MONTH;
            while month_length(year, month) as i128 <= day {
                day -= month_length(year, month) as i128;
                (year, month) = next_month(year, month);
            }
        } else {
            year = -1;
            month = LAST_MONTH;
            while day <= -(month_length(year, month) as i128) {
                day += month_length(year, month) as i128;
                (year, month) = previous_month(year, month);
            }
        }
        let day: u8 = day as u8;
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

    fn to_sec(&self) -> i128 {
        let day: i128 = if 0 < self.year {
            (0..self.year)
                .map(|year| (FIRST_MONTH..=LAST_MONTH).map(move |month| (year, month)))
                .flatten()
                .map(|(year, month)| month_length(year, month) as i128)
                .sum::<i128>()
            + (1..self.month)
                .map(|month| month_length(self.year, month) as i128)
                .sum::<i128>()
            + (self.day as i128) - 1
        } else {
            - (self.year + 1..1)
                .map(|year| (FIRST_MONTH..=LAST_MONTH).map(move |month| (year, month)))
                .flatten()
                .map(|(year, month)| month_length(year, month) as i128)
                .sum::<i128>()
            - (self.month..=LAST_MONTH)
                .map(|month| month_length(self.year, month) as i128)
                .sum::<i128>()
            + (self.day as i128) - 1
        };
        let hour: i128 = (HOURS_PER_DAY as i128) * day + (self.hour as i128);
        let min: i128 = (MINUTES_PER_HOUR as i128) * hour + (self.min as i128);
        let sec: i128 = (SECONDS_PER_MINUTE as i128) * min + (self.sec as i128);
        sec
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}/{} {}:{}:{}.{:09}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.min,
            self.sec,
            self.nsec,
        )
    }
}

fn is_leap_year(year: i128) -> bool {
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

fn month_length(year: i128, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) {
            29
        } else {
            28
        },
        month => panic!("month == {}!", month),
    }
}

fn next_month(year: i128, month: u8) -> (i128, u8) {
    match month {
        1 => (year, 2),
        2 => (year, 3),
        3 => (year, 4),
        4 => (year, 5),
        5 => (year, 6),
        6 => (year, 7),
        7 => (year, 8),
        8 => (year, 9),
        9 => (year, 10),
        10 => (year, 11),
        11 => (year, 12),
        12 => (year + 1, 1),
        month => panic!("month == {}!", month),
    }
}

fn previous_month(year: i128, month: u8) -> (i128, u8) {
    match month {
        1 => (year - 1, 12),
        2 => (year, 1),
        3 => (year, 2),
        4 => (year, 3),
        5 => (year, 4),
        6 => (year, 5),
        7 => (year, 6),
        8 => (year, 7),
        9 => (year, 8),
        10 => (year, 9),
        11 => (year, 10),
        12 => (year, 11),
        month => panic!("month == {}!", month),
    }
}

