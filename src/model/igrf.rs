use crate::model::Model;
use crate::num::{Float, NumInto};

const IGRF_EPOCH_INTERVAL: f64 = 5.0;
const IGRF_START: f64 = 1900.0;
const IGRF_END: f64 = 2025.0;
const IGRF_N_1900: usize = 10;
const IGRF_N_2000: usize = 13;
const IGRF_COF_G: [[f64; 26]; 104] = include!(concat!(env!("OUT_DIR"), "/IGRF_COF_G"));
const IGRF_COF_H: [[f64; 26]; 104] = include!(concat!(env!("OUT_DIR"), "/IGRF_COF_H"));

#[inline]
fn nm_to_index(n: usize, m: usize) -> usize {
    n * (n + 1) / 2 + m - 1
}

#[inline]
fn year_to_index(t: f64) -> usize {
    let v = ((t - IGRF_START) / IGRF_EPOCH_INTERVAL).floor();
    unsafe { v.try_into_unchecked() }
}

#[inline]
fn index_to_year(i: usize) -> f64 {
    unsafe { IGRF_EPOCH_INTERVAL * i.try_into_unchecked() + IGRF_START }
}

pub struct IGRF {
    deg: usize,
    t0: f64,
    t: f64,
    inner: [[f64; 4]; 104],
}

impl Model for IGRF {
    fn is_valid(t: f64) -> bool {
        IGRF_START <= t && t <= IGRF_END
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
        let i = nm_to_index(n, m);
        self.inner[i][0]
    }

    fn h(&self, n: usize, m: usize) -> f64 {
        let i = nm_to_index(n, m);
        self.inner[i][1]
    }

    fn g_sv(&self, n: usize, m: usize) -> f64 {
        let i = nm_to_index(n, m);
        self.inner[i][2]
    }

    fn h_sv(&self, n: usize, m: usize) -> f64 {
        let i = nm_to_index(n, m);
        self.inner[i][3]
    }
}

impl IGRF {
    pub fn new(decimal: f64) -> Option<Self> {
        if !IGRF::is_valid(decimal) {
            return None;
        }

        let n = if decimal < 2000.0 {
            IGRF_N_1900
        } else {
            IGRF_N_2000
        };

        let iy = year_to_index(decimal);
        let t0 = index_to_year(iy);
        let inner = IGRF::build(t0, iy, n);

        Some(IGRF {
            deg: n,
            t0,
            t: decimal,
            inner,
        })
    }

    #[inline]
    fn build(t0: f64, iy: usize, n: usize) -> [[f64; 4]; 104] {
        let mut inner = [[0.0; 4]; 104];

        if t0 == (IGRF_END - IGRF_EPOCH_INTERVAL) {
            let il = IGRF_COF_G[0].len() - 1;

            for j in 1..=n {
                for i in 0..=j {
                    let ii = nm_to_index(j, i);

                    let g = IGRF_COF_G[ii][iy];
                    let h = IGRF_COF_H[ii][iy];
                    let g_sv = IGRF_COF_G[ii][il];
                    let h_sv = IGRF_COF_H[ii][il];

                    inner[ii] = [g, h, g_sv, h_sv];
                }
            }
        } else {
            for j in 1..=n {
                for i in 0..=j {
                    let ii = nm_to_index(j, i);

                    let g = IGRF_COF_G[ii][iy];
                    let h = IGRF_COF_H[ii][iy];
                    let gl = IGRF_COF_G[ii][iy + 1];
                    let hl = IGRF_COF_H[ii][iy + 1];

                    let g_sv = (gl - g) / 5.0;
                    let h_sv = (hl - h) / 5.0;

                    inner[ii] = [g, h, g_sv, h_sv];
                }
            }
        }

        inner
    }
}
