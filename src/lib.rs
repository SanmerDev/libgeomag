pub use crate::datetime::DateTime;
use crate::field::MagneticField;
use crate::location::GeocentricLocation;
pub use crate::location::GeodeticLocation;
use crate::model::{Gauss, IGRF, WMM};
use crate::polynomial::lpmv;

mod datetime;
mod field;
mod location;
mod model;
mod polynomial;
pub mod util;

#[derive(Default, Debug)]
struct Prime {
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
}

impl From<Prime> for MagneticField {
    fn from(p: Prime) -> Self {
        let h = (p.x.powi(2) + p.y.powi(2)).sqrt();
        let f = (h.powi(2) + p.z.powi(2)).sqrt();
        let i = (p.z / h).atan();
        let d = (p.y / p.x).atan();

        let dh = (p.x * p.dx + p.y * p.dy) / h;
        let df = (p.x * p.dx + p.y * p.dy + p.z * p.dz) / f;
        let di = (h * p.dz - p.z * dh) / f.powi(2);
        let dd = (p.x * p.dy - p.y * p.dx) / h.powi(2);

        MagneticField {
            x: p.x,
            x_dot: p.dx,
            y: p.y,
            y_dot: p.dy,
            z: p.z,
            z_dot: p.dz,
            h,
            h_dot: dh,
            f,
            f_dot: df,
            d,
            d_dot: dd,
            i,
            i_dot: di,
        }
    }
}

pub struct Geomag<T: Gauss> {
    gauss: T,
    location: GeocentricLocation,
}

impl<T: Gauss> Geomag<T> {
    fn new(g: T, l: GeocentricLocation) -> Self {
        Geomag {
            gauss: g,
            location: l,
        }
    }

    fn lpmn(&self, n: usize, m: usize, z: f64) -> f64 {
        let mf = m as f64;
        let pnm = lpmv(n, m, z) * (-1.0_f64).powf(mf);

        if m > 0 {
            let mut d = 1.0;
            for i in (n - m + 1)..=(n + m) {
                d *= i as f64;
            }
            let c = 1. / d;
            pnm * (2.0 * c).sqrt()
        } else {
            pnm
        }
    }

    fn xyz_prime(&self, n: usize) -> Prime {
        let mut prime = Prime::default();

        let r = self.location.radius;
        let p = self.location.latitude;
        let l = self.location.longitude;
        let a = 6371200_f64;

        let ps = p.sin();
        let pc = p.cos();

        for n in 1..=n {
            let nf = n as f64;
            let f = (a / r).powf(nf + 2.0);

            for m in 0..=n {
                let mf = m as f64;
                let mlc = (mf * l).cos();
                let mls = (mf * l).sin();

                let pmn = self.lpmn(n, m, ps);
                let pmn1 = self.lpmn(n + 1, m, ps);
                let dmn = (nf + 1.0) * p.tan() * pmn
                    - ((nf + 1.0).powi(2) - mf.powi(2)).sqrt() / p.cos() * pmn1;

                let gcl = self.gauss.g(n, m) * mlc;
                let gsl = self.gauss.g(n, m) * mls;
                let hcl = self.gauss.h(n, m) * mlc;
                let hsl = self.gauss.h(n, m) * mls;

                prime.x += -f * (gcl + hsl) * dmn;
                prime.y += (f / pc) * mf * (gsl - hcl) * pmn;
                prime.z += -f * (nf + 1.0) * (gcl + hsl) * pmn;

                let d_gcl = self.gauss.dg(n, m) * mlc;
                let d_gsl = self.gauss.dg(n, m) * mls;
                let d_hcl = self.gauss.dh(n, m) * mlc;
                let d_hsl = self.gauss.dh(n, m) * mls;

                prime.dx += (-f) * (d_gcl + d_hsl) * dmn;
                prime.dy += (f / pc) * mf * (d_gsl - d_hcl) * pmn;
                prime.dz += -f * (nf + 1.0) * (d_gcl + d_hsl) * pmn;
            }
        }

        prime
    }

    fn xyz(&self, n: usize) -> Prime {
        let mut xyz = Prime::default();
        let prime = self.xyz_prime(n);

        let p1 = self.location.latitude;
        let p = self.location.inner.latitude;

        let sin_p = (p1 - p).sin();
        let cos_p = (p1 - p).cos();

        xyz.x = prime.x * cos_p - prime.z * sin_p;
        xyz.y = prime.y;
        xyz.z = prime.x * sin_p + prime.z * cos_p;

        xyz.dx = prime.dx * cos_p - prime.dz * sin_p;
        xyz.dy = prime.dy;
        xyz.dz = prime.dx * sin_p + prime.dz * cos_p;

        xyz
    }
}

impl Geomag<()> {
    pub fn wmm(location: GeodeticLocation, datetime: DateTime) -> Option<MagneticField> {
        if !WMM::is_valid(datetime.decimal) {
            return None;
        }

        let wmm = WMM::new(datetime.decimal);
        let loc = location.into();
        let n = wmm.deg;

        let mag = Geomag::new(wmm, loc);
        let xyz = mag.xyz(n);

        Some(xyz.into())
    }

    pub fn igrf(location: GeodeticLocation, datetime: DateTime) -> Option<MagneticField> {
        if !IGRF::is_valid(datetime.decimal) {
            return None;
        }

        let igrf = IGRF::new(datetime.decimal);
        let loc = location.into();
        let n = igrf.deg;

        let mag = Geomag::new(igrf, loc);
        let xyz = mag.xyz(n);

        Some(xyz.into())
    }
}
