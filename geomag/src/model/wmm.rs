use crate::model::Model;

const WMM_START: f64 = 2020.0;
const WMM_END: f64 = 2025.0;
const WMM_T0: f64 = WMM_START;
const WMM_N: usize = 12;
const WMM_COF: [[f64; 4]; 90] = include!(concat!(env!("OUT_DIR"), "/WMM_COF"));

#[inline]
fn index_for_nm(n: usize, m: usize) -> usize {
    n * (n + 1) / 2 + m - 1
}

pub struct WMM {
    deg: usize,
    t0: f64,
    t: f64,
    inner: [[f64; 4]; 90],
}

impl Model for WMM {
    fn is_valid(t: f64) -> bool {
        WMM_START < t && t < WMM_END
    }

    fn deg(&self) -> usize {
        self.deg
    }

    fn t0(&self) -> f64 {
        self.t0
    }

    fn t(&self) -> f64 {
        self.t
    }

    fn g(&self, n: usize, m: usize) -> f64 {
        let i = index_for_nm(n, m);
        self.inner[i][0]
    }

    fn h(&self, n: usize, m: usize) -> f64 {
        let i = index_for_nm(n, m);
        self.inner[i][1]
    }

    fn g_sv(&self, n: usize, m: usize) -> f64 {
        let i = index_for_nm(n, m);
        self.inner[i][2]
    }

    fn h_sv(&self, n: usize, m: usize) -> f64 {
        let i = index_for_nm(n, m);
        self.inner[i][3]
    }
}

impl WMM {
    pub fn new(decimal: f64) -> Option<Self> {
        if !WMM::is_valid(decimal) {
            return None;
        }

        let wmm = WMM {
            deg: WMM_N,
            t0: WMM_T0,
            t: decimal,
            inner: WMM_COF,
        };

        Some(wmm)
    }
}
