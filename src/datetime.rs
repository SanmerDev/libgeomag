use crate::num::NumFrom;

macro_rules! is_valid {
    ($value:expr, $min:expr, $max:expr) => {
        if $value < $min || $value > $max {
            return None;
        }
    };
}

#[derive(Debug, Copy, Clone)]
pub struct DateTime {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub min: u32,
    pub sec: u32,
}

impl DateTime {
    #[allow(unused_comparisons)]
    pub fn new(year: u32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> Option<Self> {
        is_valid!(year, 1_000, 10_000);
        is_valid!(month, 1, 12);
        is_valid!(day, 1, 31);
        is_valid!(hour, 0, 24);
        is_valid!(min, 0, 60);
        is_valid!(sec, 0, 60);

        Some(DateTime {
            year,
            month,
            day,
            hour,
            min,
            sec,
        })
    }

    pub fn is_leap_year(&self) -> bool {
        is_leap_year(self.year)
    }

    pub fn days(&self) -> u32 {
        let leap = self.is_leap_year();
        let days: u32 = (1..self.month)
            .map(|m| unsafe { days_of_month(m, leap) })
            .sum();

        days + self.day
    }

    pub fn decimal(&self) -> f64 {
        unsafe {
            let all_days = if self.is_leap_year() {
                366_f64
            } else {
                365_f64
            };
            let all_d = all_days * 24_f64 * 60_f64 * 60_f64;

            let days = self.days() - 1;
            let days_d = f64::from_unchecked(days) * 24_f64 * 60_f64 * 60_f64;
            let hour_d = f64::from_unchecked(self.hour) * 60_f64 * 60_f64;
            let min_d = f64::from_unchecked(self.min) * 60_f64;
            let now_d = days_d + hour_d + min_d + f64::from_unchecked(self.sec);

            let d = now_d / all_d;
            f64::from_unchecked(self.year) + d
        }
    }
}

#[inline]
fn is_leap_year(year: u32) -> bool {
    if year % 4 != 0 {
        return false;
    }

    if year % 100 == 0 && year % 400 != 0 {
        return false;
    }

    true
}

#[inline]
unsafe fn days_of_month(month: u32, leap: bool) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if leap {
                29
            } else {
                28
            }
        }
        _ => unreachable!(),
    }
}
