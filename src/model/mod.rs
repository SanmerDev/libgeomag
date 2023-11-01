pub use igrf::IGRF;
pub use wmm::WMM;

mod igrf;
mod wmm;

pub trait Gauss {
    fn index_for_nm(n: usize, m: usize) -> usize {
        n * (n + 1) / 2 + m - 1
    }

    fn g(&self, n: usize, m: usize) -> f64;
    fn h(&self, n: usize, m: usize) -> f64;
    fn dg(&self, n: usize, m: usize) -> f64;
    fn dh(&self, n: usize, m: usize) -> f64;
}

impl Gauss for () {
    fn g(&self, _: usize, _: usize) -> f64 {
        unimplemented!();
    }

    fn h(&self, _: usize, _: usize) -> f64 {
        unimplemented!();
    }

    fn dg(&self, _: usize, _: usize) -> f64 {
        unimplemented!();
    }

    fn dh(&self, _: usize, _: usize) -> f64 {
        unimplemented!();
    }
}
