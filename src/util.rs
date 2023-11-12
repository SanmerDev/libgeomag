use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Timelike};
use std::f64::consts::PI;

pub trait MathExt {
    fn to_rad(&self) -> f64;
    fn to_deg(&self) -> f64;
}

impl MathExt for f64 {
    fn to_rad(&self) -> f64 {
        (self * PI) / 180.0
    }

    fn to_deg(&self) -> f64 {
        (self * 180.0) / PI
    }
}

pub trait DateTimeExt {
    fn to_decimal_years(&self) -> f64;
}

impl<Tz: TimeZone> DateTimeExt for DateTime<Tz> {
    fn to_decimal_years(&self) -> f64 {
        let t_year = self.year();
        let year_days = NaiveDate::from_ymd_opt(t_year, 12, 31).unwrap().ordinal();
        let t_day = (self.ordinal() - 1) as f64;

        let time = self.time();
        let t_seconds = time.num_seconds_from_midnight() + time.nanosecond() / 1_000_000_000;
        t_year as f64 + (t_day + t_seconds as f64 / 86_400.0) / year_days as f64
    }
}
