pub trait IntFrom<T>: Sized {
    unsafe fn from_unchecked(v: T) -> Self;
    fn from(v: T) -> Option<Self>;
}

impl IntFrom<i32> for u32 {
    unsafe fn from_unchecked(v: i32) -> Self {
        v as u32
    }

    fn from(v: i32) -> Option<Self> {
        match v {
            v if v >= 0 => Some(unsafe { Self::from_unchecked(v) }),
            _ => None,
        }
    }
}

impl IntFrom<f64> for usize {
    unsafe fn from_unchecked(v: f64) -> Self {
        v as usize
    }

    fn from(v: f64) -> Option<Self> {
        match v {
            v if v >= 0.0 && v <= (usize::MAX as f64) => Some(unsafe { Self::from_unchecked(v) }),
            _ => None,
        }
    }
}
