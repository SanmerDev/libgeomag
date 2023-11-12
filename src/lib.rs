pub use crate::datetime::DateTime;
pub use crate::field::MagneticField;
use crate::location::GeocentricLocation;
pub use crate::location::GeodeticLocation;
use crate::model::{Gauss, Model};
pub use crate::model::{IGRF, WMM};
use crate::polynomial::lpmv;

mod datetime;
mod field;
mod location;
mod model;
mod polynomial;
pub mod util;

#[derive(Default)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
}

impl From<Vector> for MagneticField {
    fn from(v: Vector) -> Self {
        let h = (v.x.powi(2) + v.y.powi(2)).sqrt();
        let f = (h.powi(2) + v.z.powi(2)).sqrt();
        let d = (v.y / v.x).atan();
        let i = (v.z / h).atan();

        let dh = (v.x * v.dx + v.y * v.dy) / h;
        let df = (v.x * v.dx + v.y * v.dy + v.z * v.dz) / f;
        let dd = (v.x * v.dy - v.y * v.dx) / h.powi(2);
        let di = (h * v.dz - v.z * dh) / f.powi(2);

        MagneticField {
            x: v.x,
            x_dot: v.dx,
            y: v.y,
            y_dot: v.dy,
            z: v.z,
            z_dot: v.dz,
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

struct Calculator<T: Gauss> {
    deg: usize,
    gauss: T,
    location: GeocentricLocation,
}

impl<T: Gauss> Calculator<T> {
    fn new(n: usize, g: T, l: GeocentricLocation) -> Self {
        Calculator {
            deg: n,
            gauss: g,
            location: l,
        }
    }

    fn lpmn(&self, n: usize, m: usize, z: f64) -> f64 {
        let mf = m as f64;
        let pnm = (-1.0_f64).powf(mf) * lpmv(n, m, z);

        if m > 0 {
            let mut d = 1.0;
            for i in (n - m + 1)..=(n + m) {
                d *= i as f64;
            }

            pnm * (2.0 * (1.0 / d)).sqrt()
        } else {
            pnm
        }
    }

    fn xyz_prime(&self) -> Vector {
        let mut vector = Vector::default();

        let r = self.location.radius;
        let p = self.location.latitude;
        let l = self.location.longitude;
        let a = 6371200.0_f64;
        let ps = p.sin();
        let pc = p.cos();

        for n in 1..=self.deg {
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

                vector.x += -f * (gcl + hsl) * dmn;
                vector.y += (f / pc) * mf * (gsl - hcl) * pmn;
                vector.z += -f * (nf + 1.0) * (gcl + hsl) * pmn;

                let d_gcl = self.gauss.dg(n, m) * mlc;
                let d_gsl = self.gauss.dg(n, m) * mls;
                let d_hcl = self.gauss.dh(n, m) * mlc;
                let d_hsl = self.gauss.dh(n, m) * mls;

                vector.dx += (-f) * (d_gcl + d_hsl) * dmn;
                vector.dy += (f / pc) * mf * (d_gsl - d_hcl) * pmn;
                vector.dz += -f * (nf + 1.0) * (d_gcl + d_hsl) * pmn;
            }
        }

        vector
    }

    fn xyz(&self) -> Vector {
        let mut xyz = Vector::default();
        let vector = self.xyz_prime();

        let p1 = self.location.latitude;
        let p = self.location.inner.latitude;
        let sin_p = (p1 - p).sin();
        let cos_p = (p1 - p).cos();

        xyz.x = vector.x * cos_p - vector.z * sin_p;
        xyz.y = vector.y;
        xyz.z = vector.x * sin_p + vector.z * cos_p;

        xyz.dx = vector.dx * cos_p - vector.dz * sin_p;
        xyz.dy = vector.dy;
        xyz.dz = vector.dx * sin_p + vector.dz * cos_p;

        xyz
    }
}

pub trait ModelExt {
    fn single(self, location: GeodeticLocation) -> MagneticField;
}

impl<T: Model> ModelExt for T {
    fn single(self, location: GeodeticLocation) -> MagneticField {
        let l = location.into();
        let mag = Calculator::new(self.deg(), self, l);
        let xyz = mag.xyz();
        xyz.into()
    }
}
