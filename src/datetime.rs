use std::fmt::{Display, Formatter};
use std::ops::Deref;

use chrono::{DateTime as DT, TimeZone, Utc};

use crate::util::DateTimeExt;

#[derive(Copy, Clone)]
pub struct DateTime {
    inner: DT<Utc>,
    pub decimal: f64,
}

impl DateTime {
    pub fn new(year: i32, month: i32, day: i32, hour: i32, min: i32, sec: i32) -> Self {
        let dt = Utc
            .with_ymd_and_hms(
                year,
                month as u32,
                day as u32,
                hour as u32,
                min as u32,
                sec as u32,
            )
            .unwrap();

        DateTime {
            inner: dt,
            decimal: dt.to_decimal_years(),
        }
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Deref for DateTime {
    type Target = DT<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
