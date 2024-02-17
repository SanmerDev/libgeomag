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
