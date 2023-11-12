use crate::model::Model;

const WMM_START: f64 = 2020.0;
const WMM_END: f64 = 2025.0;
const WMM_T0: f64 = WMM_START;
const WMM_N: usize = 12;
const WMM_COEFFICIENTS: [[f64; 4]; 90] = [
    [-29404.5, 0.0, 6.7, 0.0],
    [-1450.7, 4652.9, 7.7, -25.1],
    [-2500.0, 0.0, -11.5, 0.0],
    [2982.0, -2991.6, -7.1, -30.2],
    [1676.8, -734.8, -2.2, -23.9],
    [1363.9, 0.0, 2.8, 0.0],
    [-2381.0, -82.2, -6.2, 5.7],
    [1236.2, 241.8, 3.4, -1.0],
    [525.7, -542.9, -12.2, 1.1],
    [903.1, 0.0, -1.1, 0.0],
    [809.4, 282.0, -1.6, 0.2],
    [86.2, -158.4, -6.0, 6.9],
    [-309.4, 199.8, 5.4, 3.7],
    [47.9, -350.1, -5.5, -5.6],
    [-234.4, 0.0, -0.3, 0.0],
    [363.1, 47.7, 0.6, 0.1],
    [187.8, 208.4, -0.7, 2.5],
    [-140.7, -121.3, 0.1, -0.9],
    [-151.2, 32.2, 1.2, 3.0],
    [13.7, 99.1, 1.0, 0.5],
    [65.9, 0.0, -0.6, 0.0],
    [65.6, -19.1, -0.4, 0.1],
    [73.0, 25.0, 0.5, -1.8],
    [-121.5, 52.7, 1.4, -1.4],
    [-36.2, -64.4, -1.4, 0.9],
    [13.5, 9.0, -0.0, 0.1],
    [-64.7, 68.1, 0.8, 1.0],
    [80.6, 0.0, -0.1, 0.0],
    [-76.8, -51.4, -0.3, 0.5],
    [-8.3, -16.8, -0.1, 0.6],
    [56.5, 2.3, 0.7, -0.7],
    [15.8, 23.5, 0.2, -0.2],
    [6.4, -2.2, -0.5, -1.2],
    [-7.2, -27.2, -0.8, 0.2],
    [9.8, -1.9, 1.0, 0.3],
    [23.6, 0.0, -0.1, 0.0],
    [9.8, 8.4, 0.1, -0.3],
    [-17.5, -15.3, -0.1, 0.7],
    [-0.4, 12.8, 0.5, -0.2],
    [-21.1, -11.8, -0.1, 0.5],
    [15.3, 14.9, 0.4, -0.3],
    [13.7, 3.6, 0.5, -0.5],
    [-16.5, -6.9, 0.0, 0.4],
    [-0.3, 2.8, 0.4, 0.1],
    [5.0, 0.0, -0.1, 0.0],
    [8.2, -23.3, -0.2, -0.3],
    [2.9, 11.1, -0.0, 0.2],
    [-1.4, 9.8, 0.4, -0.4],
    [-1.1, -5.1, -0.3, 0.4],
    [-13.3, -6.2, -0.0, 0.1],
    [1.1, 7.8, 0.3, -0.0],
    [8.9, 0.4, -0.0, -0.2],
    [-9.3, -1.5, -0.0, 0.5],
    [-11.9, 9.7, -0.4, 0.2],
    [-1.9, 0.0, 0.0, 0.0],
    [-6.2, 3.4, -0.0, -0.0],
    [-0.1, -0.2, -0.0, 0.1],
    [1.7, 3.5, 0.2, -0.3],
    [-0.9, 4.8, -0.1, 0.1],
    [0.6, -8.6, -0.2, -0.2],
    [-0.9, -0.1, -0.0, 0.1],
    [1.9, -4.2, -0.1, -0.0],
    [1.4, -3.4, -0.2, -0.1],
    [-2.4, -0.1, -0.1, 0.2],
    [-3.9, -8.8, -0.0, -0.0],
    [3.0, 0.0, -0.0, 0.0],
    [-1.4, -0.0, -0.1, -0.0],
    [-2.5, 2.6, -0.0, 0.1],
    [2.4, -0.5, 0.0, 0.0],
    [-0.9, -0.4, -0.0, 0.2],
    [0.3, 0.6, -0.1, -0.0],
    [-0.7, -0.2, 0.0, 0.0],
    [-0.1, -1.7, -0.0, 0.1],
    [1.4, -1.6, -0.1, -0.0],
    [-0.6, -3.0, -0.1, -0.1],
    [0.2, -2.0, -0.1, 0.0],
    [3.1, -2.6, -0.1, -0.0],
    [-2.0, 0.0, 0.0, 0.0],
    [-0.1, -1.2, -0.0, -0.0],
    [0.5, 0.5, -0.0, 0.0],
    [1.3, 1.3, 0.0, -0.1],
    [-1.2, -1.8, -0.0, 0.1],
    [0.7, 0.1, -0.0, -0.0],
    [0.3, 0.7, 0.0, 0.0],
    [0.5, -0.1, -0.0, -0.0],
    [-0.2, 0.6, 0.0, 0.1],
    [-0.5, 0.2, -0.0, -0.0],
    [0.1, -0.9, -0.0, -0.0],
    [-1.1, -0.0, -0.0, 0.0],
    [-0.3, 0.5, -0.1, -0.1],
];

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
            inner: WMM_COEFFICIENTS,
        };

        Some(wmm)
    }
}
