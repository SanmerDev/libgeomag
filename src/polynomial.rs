///## References
///* [Zhang, Jin, “Computation of Special Functions”, John Wiley and Sons, Inc, 1996](https://people.sc.fsu.edu/~jburkardt/f77_src/special_functions/special_functions.html)
///
pub fn lpmv(n: usize, m: usize, x: f64) -> f64 {
    let nf = n as f64;
    let mf = m as f64;
    let mut c0 = 1.0;

    if m != 0 {
        let mut rg = nf * (nf + mf);
        for j in 1..m {
            let jf = j as f64;
            rg *= nf * nf - jf * jf;
        }

        let xq = (1.0 - x * x).sqrt();
        let mut r0 = 1.0;
        for j in 1..=m {
            let jf = j as f64;
            r0 *= 0.5 * xq / jf;
        }

        c0 = r0 * rg;
    }

    let mut pmv = 1.0;
    let mut r = 1.0;

    for k in 1..=(n - m) {
        let kf = k as f64;
        r *= 0.5 * (mf - nf + kf - 1.0) * (nf + mf + kf) / (kf * (kf + mf)) * (1.0 + x);
        pmv += r;
    }

    (-1.0_f64).powf(nf) * c0 * pmv
}
