pub use igrf::IGRF;
pub use wmm::WMM;

mod igrf;
mod wmm;

pub trait Model {
    fn is_valid(t: f64) -> bool;
    fn deg(&self) -> usize;
    fn t0(&self) -> f64;
    fn t(&self) -> f64;
    fn g(&self, n: usize, m: usize) -> f64;
    fn h(&self, n: usize, m: usize) -> f64;
    fn g_sv(&self, n: usize, m: usize) -> f64;
    fn h_sv(&self, n: usize, m: usize) -> f64;
}

pub trait Gauss {
    fn g(&self, n: usize, m: usize) -> f64;
    fn h(&self, n: usize, m: usize) -> f64;
    fn dg(&self, n: usize, m: usize) -> f64;
    fn dh(&self, n: usize, m: usize) -> f64;
}

impl<T: Model> Gauss for T {
    #[inline]
    fn g(&self, n: usize, m: usize) -> f64 {
        self.g(n, m) + (self.t() - self.t0()) * self.g_sv(n, m)
    }

    #[inline]
    fn h(&self, n: usize, m: usize) -> f64 {
        self.h(n, m) + (self.t() - self.t0()) * self.h_sv(n, m)
    }

    #[inline]
    fn dg(&self, n: usize, m: usize) -> f64 {
        self.g_sv(n, m)
    }

    #[inline]
    fn dh(&self, n: usize, m: usize) -> f64 {
        self.h_sv(n, m)
    }
}
