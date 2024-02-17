use num_traits::Float;
use num_traits::FromPrimitive;

use crate::from_usize;
use crate::model::Model;

const IGRF_EPOCH_INTERVAL: f64 = 5.0;
const IGRF_START: f64 = 1900.0;
const IGRF_END: f64 = 2025.0;
const IGRF_N_1900: usize = 10;
const IGRF_N_2000: usize = 13;
const IGRF_COF_G: [[f64; 26]; 104] = include!(concat!(env!("OUT_DIR"), "/IGRF_COF_G"));
const IGRF_COF_H: [[f64; 26]; 104] = include!(concat!(env!("OUT_DIR"), "/IGRF_COF_H"));

#[inline]
fn index_for_nm(n: usize, m: usize) -> usize {
    n * (n + 1) / 2 + m - 1
}

#[inline]
fn index_for_year(t: f64) -> usize {
    let v = ((t - IGRF_START) / IGRF_EPOCH_INTERVAL).floor();
    unsafe { usize::from_f64(v).unwrap_unchecked() }
}

#[inline]
fn year_from_index(i: usize) -> f64 {
    IGRF_EPOCH_INTERVAL * (from_usize::<f64>(i)) + IGRF_START
}

pub struct IGRF {
    deg: usize,
    t0: f64,
    t: f64,
    inner: [[f64; 4]; 104],
}

impl Model for IGRF {
    fn is_valid(t: f64) -> bool {
        IGRF_START < t && t < IGRF_END
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
        let iy = index_for_year(decimal);
        let t0 = year_from_index(iy);
        let inner = IGRF::build(t0, iy, n);

        let igrf = IGRF {
            deg: n,
            t0,
            t: decimal,
            inner,
        };

        Some(igrf)
    }

    #[inline]
    fn build(t0: f64, iy: usize, n: usize) -> [[f64; 4]; 104] {
        let mut inner = [[0.0; 4]; 104];

        if t0 == (IGRF_END - IGRF_EPOCH_INTERVAL) {
            let il = IGRF_COF_G[0].len() - 1;

            for j in 1..=n {
                for i in 0..=j {
                    let ii = index_for_nm(j, i);

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
                    let ii = index_for_nm(j, i);

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
