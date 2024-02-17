pub use float::Float;

mod float;

pub trait NumInto<T>: Sized {
    unsafe fn to_unchecked(self) -> T;
}

impl NumInto<f64> for usize {
    unsafe fn to_unchecked(self) -> f64 {
        self as f64
    }
}

impl NumInto<f64> for u32 {
    unsafe fn to_unchecked(self) -> f64 {
        self as f64
    }
}

impl NumInto<u32> for i32 {
    unsafe fn to_unchecked(self) -> u32 {
        self as u32
    }
}

impl NumInto<usize> for f64 {
    unsafe fn to_unchecked(self) -> usize {
        self as usize
    }
}
