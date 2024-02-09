#[cfg(feature = "chrono")]
pub use crate::datetime::DateTime;
pub use crate::field::MagneticField;

pub use crate::location::GeodeticLocation;
#[cfg(feature = "igrf")]
pub use crate::model::IGRF;
#[cfg(feature = "wmm")]
pub use crate::model::WMM;

use crate::location::GeocentricLocation;
use crate::model::{Gauss, Model};
use crate::polynomial::lpmv;

use num_traits::FromPrimitive;

#[cfg(feature = "chrono")]
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

struct Calculator<'a, T> {
    deg: usize,
    gauss: &'a T,
    location: &'a GeocentricLocation,
}

impl<'a, T: Gauss> Calculator<'a, T> {
    fn new(n: usize, g: &'a T, l: &'a GeocentricLocation) -> Self {
        Calculator {
            deg: n,
            gauss: g,
            location: l,
        }
    }

    #[inline]
    fn lpmn(&self, n: usize, m: usize, z: f64) -> f64 {
        let m_f = from_usize(m);
        let pnm = (-1.0_f64).powf(m_f) * lpmv(n, m, z);

        if m > 0 {
            let mut d = 1.0;
            for i in (n - m + 1)..=(n + m) {
                d *= from_usize::<f64>(i);
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
            let n_f = from_usize::<f64>(n);
            let f = (a / r).powf(n_f + 2.0);

            for m in 0..=n {
                let m_f = from_usize::<f64>(m);
                let m_lc = (m_f * l).cos();
                let m_ls = (m_f * l).sin();

                let pmn = self.lpmn(n, m, ps);
                let pmn1 = self.lpmn(n + 1, m, ps);
                let dmn = (n_f + 1.0) * p.tan() * pmn
                    - ((n_f + 1.0).powi(2) - m_f.powi(2)).sqrt() / p.cos() * pmn1;

                let gcl = self.gauss.g(n, m) * m_lc;
                let gsl = self.gauss.g(n, m) * m_ls;
                let hcl = self.gauss.h(n, m) * m_lc;
                let hsl = self.gauss.h(n, m) * m_ls;

                vector.x += -f * (gcl + hsl) * dmn;
                vector.y += (f / pc) * m_f * (gsl - hcl) * pmn;
                vector.z += -f * (n_f + 1.0) * (gcl + hsl) * pmn;

                let d_gcl = self.gauss.dg(n, m) * m_lc;
                let d_gsl = self.gauss.dg(n, m) * m_ls;
                let d_hcl = self.gauss.dh(n, m) * m_lc;
                let d_hsl = self.gauss.dh(n, m) * m_ls;

                vector.dx += (-f) * (d_gcl + d_hsl) * dmn;
                vector.dy += (f / pc) * m_f * (d_gsl - d_hcl) * pmn;
                vector.dz += -f * (n_f + 1.0) * (d_gcl + d_hsl) * pmn;
            }
        }

        vector
    }

    fn xyz(&self) -> Vector {
        let mut xyz = Vector::default();
        let vector = self.xyz_prime();

        let p1 = self.location.latitude;
        let p = self.location.geodetic.latitude;
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

pub trait Geomag {
    fn at_location(self, l: &GeodeticLocation) -> MagneticField;
}

impl<T> Geomag for &T
where
    T: Model,
{
    fn at_location(self, l: &GeodeticLocation) -> MagneticField {
        let l = l.into();
        let mag = Calculator::new(self.deg(), self, &l);
        let xyz = mag.xyz();
        xyz.into()
    }
}

#[inline]
pub(crate) fn from_usize<T: FromPrimitive>(v: usize) -> T {
    unsafe { T::from_usize(v).unwrap_unchecked() }
}
