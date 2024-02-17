#[cfg(feature = "chrono")]
use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Timelike};
use core::f64::consts::PI;

pub trait MathExt {
    fn to_rad(&self) -> f64;
    fn to_deg(&self) -> f64;
}

impl MathExt for f64 {
    #[inline]
    fn to_rad(&self) -> f64 {
        (self * PI) / 180.0
    }

    #[inline]
    fn to_deg(&self) -> f64 {
        (self * 180.0) / PI
    }
}

#[cfg(feature = "chrono")]
pub trait DateTimeExt {
    fn to_decimal_years(&self) -> Option<f64>;
}

#[cfg(feature = "chrono")]
impl<Tz: TimeZone> DateTimeExt for DateTime<Tz> {
    fn to_decimal_years(&self) -> Option<f64> {
        let t_year = self.year();
        let t_days = unsafe {
            NaiveDate::from_ymd_opt(t_year, 12, 31)
                .unwrap_unchecked()
                .ordinal()
        };

        let t_year = f64::from(t_year);
        let t_day = f64::from(self.ordinal() - 1);
        let t_days = f64::from(t_days);

        let time = self.time();
        let t_seconds = time.num_seconds_from_midnight() + time.nanosecond() / 1_000_000_000;
        let t_seconds = f64::from(t_seconds);

        Some(t_year + (t_day + t_seconds / 86_400.0) / t_days)
    }
}
