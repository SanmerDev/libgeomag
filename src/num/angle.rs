use core::f64::consts::PI;
use core::fmt;

#[derive(Copy, Clone)]
pub struct Radian(f64);

impl Radian {
    #[inline]
    pub fn deg(&self) -> Degree {
        Degree(self.0.to_degrees())
    }
}

#[derive(Copy, Clone)]
pub struct Degree(f64);

impl Degree {
    #[inline]
    pub fn rad(&self) -> Radian {
        Radian(self.0.to_radians())
    }
}

pub trait Angle {
    fn rad(self) -> Radian;
    fn deg(self) -> Degree;
}

impl Angle for f64 {
    #[inline]
    fn rad(self) -> Radian {
        Radian(self)
    }

    #[inline]
    fn deg(self) -> Degree {
        Degree(self)
    }
}

macro_rules! impl_f {
    ($type:ty) => {
        impl $type {
            pub fn f(&self) -> f64 {
                self.0
            }
        }
    };
}

impl_f!(Radian);
impl_f!(Degree);

macro_rules! impl_display {
    ($type:ty) => {
        impl fmt::Debug for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(&self.0, f)
            }
        }

        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0, f)
            }
        }
    };
}

impl_display!(Radian);
impl_display!(Degree);
