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
    pub fn new(unix_epoch_sec: u32, nsec: u32) -> Self {
        let sec_per_min = 60;
        let min_per_hour = 60;
        let hour_per_day = 24;
        let sec = unix_epoch_sec % sec_per_min;
        let unix_epoch_min = unix_epoch_sec / sec_per_min;
        let min = unix_epoch_min % min_per_hour;
        let unix_epoch_hour = unix_epoch_min / min_per_hour;
        let hour = unix_epoch_hour % hour_per_day;
        let unix_epoch_day = unix_epoch_hour / hour_per_day;
        let mut year = 1970;
        let mut month = 1;
        let mut day = unix_epoch_day;
        while day_per_month(year, month) < day {
            day -= day_per_month(year, month);
            if month < 12 {
                month += 1;
            } else if month == 12 {
                year += 1;
                month = 1;
            }
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
}

fn day_per_month(year: i32, month: u8) => u8 {
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

fn is_leap_year(year: i32) -> bool {
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

