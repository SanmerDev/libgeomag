use core::f64::consts::PI;
use core::fmt;

#[derive(Copy, Clone)]
pub struct Rad(f64);

impl Rad {
    #[inline]
    pub fn deg(&self) -> Deg {
        Deg(self.0.to_degrees())
    }
}

#[derive(Copy, Clone)]
pub struct Deg(f64);

impl Deg {
    #[inline]
    pub fn rad(&self) -> Rad {
        Rad(self.0.to_radians())
    }
}

pub trait Unit {
    fn rad(self) -> Rad;
    fn deg(self) -> Deg;
}

impl Unit for f64 {
    #[inline]
    fn rad(self) -> Rad {
        Rad(self)
    }

    #[inline]
    fn deg(self) -> Deg {
        Deg(self)
    }
}

macro_rules! impl_v {
    ($type:ty) => {
        impl $type {
            pub fn v(&self) -> f64 {
                self.0
            }
        }
    };
}

impl_v!(Rad);
impl_v!(Deg);

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

impl_display!(Rad);
impl_display!(Deg);
