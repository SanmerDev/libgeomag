use crate::num::{Float, FloatFrom};

///## References
///* [Zhang, Jin, “Computation of Special Functions”, John Wiley and Sons, Inc, 1996](https://people.sc.fsu.edu/~jburkardt/f77_src/special_functions/special_functions.html)
///
#[inline]
pub unsafe fn lpmv(n: usize, m: usize, x: f64) -> f64 {
    let n_f = f64::from_unchecked(n);
    let m_f = f64::from_unchecked(m);
    let mut c = 1.0;

    if m != 0 {
        let mut rg = n_f * (n_f + m_f);
        for j in 1..m {
            let j_f = f64::from_unchecked(j);
            rg *= n_f * n_f - j_f * j_f;
        }

        let xq = (1.0 - x * x).sqrt();
        let mut r = 1.0;
        for j in 1..=m {
            let j_f = f64::from_unchecked(j);
            r *= 0.5 * xq / j_f;
        }

        c = r * rg;
    }

    let mut pmv = 1.0;
    let mut r = 1.0;

    for k in 1..=(n - m) {
        let k_f = f64::from_unchecked(k);
        r *= 0.5 * (m_f - n_f + k_f - 1.0) * (n_f + m_f + k_f) / (k_f * (k_f + m_f)) * (1.0 + x);
        pmv += r;
    }

    (-1.0_f64).powf(n_f) * c * pmv
}
