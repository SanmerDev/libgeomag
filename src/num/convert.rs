pub trait NumFrom<T>: Sized {
    unsafe fn from_unchecked(value: T) -> Self;
}

pub trait NumInto<T>: Sized {
    unsafe fn into_unchecked(self) -> T;
}

impl<U, T> NumInto<U> for T
where
    U: NumFrom<T>,
{
    #[inline]
    unsafe fn into_unchecked(self) -> U {
        U::from_unchecked(self)
    }
}

impl NumFrom<usize> for f64 {
    #[inline]
    unsafe fn from_unchecked(value: usize) -> f64 {
        value as f64
    }
}

impl NumFrom<u32> for f64 {
    #[inline]
    unsafe fn from_unchecked(value: u32) -> f64 {
        value as f64
    }
}

impl NumFrom<i32> for u32 {
    #[inline]
    unsafe fn from_unchecked(value: i32) -> u32 {
        value as u32
    }
}

impl NumFrom<f64> for usize {
    #[inline]
    unsafe fn from_unchecked(value: f64) -> usize {
        value as usize
    }
}
