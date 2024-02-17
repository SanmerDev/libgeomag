#[cfg(feature = "libm")]
use libm::Libm;

pub trait Float: Sized {
    fn floor(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn atan(self) -> Self;
    fn sqrt(self) -> Self;
    fn powf(self, n: Self) -> Self;
    fn powi(self, n: i32) -> Self;
}

#[cfg(not(feature = "libm"))]
impl Float for f64 {
    #[inline]
    fn floor(self) -> Self {
        self.floor()
    }

    #[inline]
    fn sin(self) -> Self {
        self.sin()
    }

    #[inline]
    fn cos(self) -> Self {
        self.cos()
    }

    #[inline]
    fn tan(self) -> Self {
        self.tan()
    }

    #[inline]
    fn asin(self) -> Self {
        self.asin()
    }

    #[inline]
    fn atan(self) -> Self {
        self.atan()
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    #[inline]
    fn powf(self, n: Self) -> Self {
        self.powf(n)
    }

    #[inline]
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }
}

#[cfg(feature = "libm")]
impl Float for f64 {
    #[inline]
    fn floor(self) -> Self {
        Libm::<f64>::floor(self)
    }

    #[inline]
    fn sin(self) -> Self {
        Libm::<f64>::sin(self)
    }

    #[inline]
    fn cos(self) -> Self {
        Libm::<f64>::cos(self)
    }

    #[inline]
    fn tan(self) -> Self {
        Libm::<f64>::tan(self)
    }

    #[inline]
    fn asin(self) -> Self {
        Libm::<f64>::asin(self)
    }

    #[inline]
    fn atan(self) -> Self {
        Libm::<f64>::atan(self)
    }

    #[inline]
    fn sqrt(self) -> Self {
        Libm::<f64>::sqrt(self)
    }

    #[inline]
    fn powf(self, n: Self) -> Self {
        Libm::<f64>::pow(self, n)
    }

    #[inline]
    fn powi(self, n: i32) -> Self {
        self.powf(n as f64)
    }
}
