use std::fmt;
use std::ops::Deref;

use chrono::{DateTime as DT, LocalResult, TimeZone, Utc};
use num_traits::FromPrimitive;

use crate::util::DateTimeExt;

#[derive(Copy, Clone)]
pub struct DateTime {
    inner: DT<Utc>,
    pub decimal: f64,
}

impl DateTime {
    pub fn new(year: i32, month: i32, day: i32, hour: i32, min: i32, sec: i32) -> Option<Self> {
        let dt = Utc.with_ymd_and_hms(
            year,
            u32::from_i32(month)?,
            u32::from_i32(day)?,
            u32::from_i32(hour)?,
            u32::from_i32(min)?,
            u32::from_i32(sec)?,
        );

        let utc = match dt {
            LocalResult::Single(v) => v,
            _ => return None,
        };

        Some(DateTime {
            inner: utc,
            decimal: utc.to_decimal_years()?,
        })
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.inner.to_string())
    }
}

impl Deref for DateTime {
    type Target = DT<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
