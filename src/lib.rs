#![allow(unused_imports)]
#![cfg_attr(feature = "libm", no_std)]

pub use crate::datetime::DateTime;
pub use crate::field::MagneticField;
pub use crate::location::GeodeticLocation;

use crate::location::GeocentricLocation;
use crate::model::{Gauss, Model};
use crate::num::{Float, NumInto, Angle};
use crate::polynomial::lpmv;

mod datetime;
mod field;
mod location;
pub mod model;
pub mod num;
mod polynomial;

#[derive(Default)]
pub(crate) struct Vector {
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
            d: d.rad(),
            d_dot: dd.rad(),
            i: i.rad(),
            i_dot: di.rad(),
        }
    }
}

pub(crate) struct Calculator<'a, T>
where
    T: Gauss,
{
    deg: usize,
    gauss: &'a T,
    geocentric: &'a GeocentricLocation,
    geodetic: &'a GeodeticLocation,
}

impl<'a, T: Gauss> Calculator<'a, T> {
    pub(crate) fn new(
        deg: usize,
        gauss: &'a T,
        geocentric: &'a GeocentricLocation,
        geodetic: &'a GeodeticLocation,
    ) -> Self {
        Calculator {
            deg,
            gauss,
            geocentric,
            geodetic,
        }
    }

    #[inline]
    unsafe fn lpmn(&self, n: usize, m: usize, z: f64) -> f64 {
        let m_f = m.try_into_unchecked();
        let pnm = (-1.0_f64).powf(m_f) * lpmv(n, m, z);

        if m > 0 {
            let mut d = 1.0;
            for i in (n - m + 1)..=(n + m) {
                d *= i.try_into_unchecked();
            }

            pnm * (2.0 * (1.0 / d)).sqrt()
        } else {
            pnm
        }
    }

    unsafe fn xyz_prime(&self) -> Vector {
        let mut prime = Vector::default();

        let r = self.geocentric.radius;
        let p = self.geocentric.latitude;
        let l = self.geocentric.longitude;
        let a = 6371200.0_f64;

        let sin_p = p.sin();
        let cos_p = p.cos();

        for n in 1..=self.deg {
            let n_f = n.try_into_unchecked();
            let f = (a / r).powf(n_f + 2.0);

            for m in 0..=n {
                let m_f = m.try_into_unchecked();
                let cos_ml = (m_f * l).cos();
                let sin_ml = (m_f * l).sin();

                let pmn = self.lpmn(n, m, sin_p);
                let pmn1 = self.lpmn(n + 1, m, sin_p);
                let dmn = (n_f + 1.0) * p.tan() * pmn
                    - ((n_f + 1.0).powi(2) - m_f.powi(2)).sqrt() / p.cos() * pmn1;

                let g = self.gauss.g(n, m);
                let h = self.gauss.h(n, m);
                let g_cos_ml = g * cos_ml;
                let g_sin_ml = g * sin_ml;
                let h_cos_ml = h * cos_ml;
                let h_sin_ml = h * sin_ml;

                prime.x += -f * (g_cos_ml + h_sin_ml) * dmn;
                prime.y += (f / cos_p) * m_f * (g_sin_ml - h_cos_ml) * pmn;
                prime.z += -f * (n_f + 1.0) * (g_cos_ml + h_sin_ml) * pmn;

                let dg = self.gauss.dg(n, m);
                let dh = self.gauss.dh(n, m);
                let dg_cos_ml = dg * cos_ml;
                let dg_sin_ml = dg * sin_ml;
                let dh_cos_ml = dh * cos_ml;
                let dh_sin_ml = dh * sin_ml;

                prime.dx += (-f) * (dg_cos_ml + dh_sin_ml) * dmn;
                prime.dy += (f / cos_p) * m_f * (dg_sin_ml - dh_cos_ml) * pmn;
                prime.dz += -f * (n_f + 1.0) * (dg_cos_ml + dh_sin_ml) * pmn;
            }
        }

        prime
    }

    pub(crate) fn xyz(&self) -> Vector {
        let mut xyz = Vector::default();
        let prime = unsafe { self.xyz_prime() };

        let p1 = self.geocentric.latitude;
        let p = self.geodetic.latitude;
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

pub trait Geomag {
    fn at_location(self, geodetic: &GeodeticLocation) -> MagneticField;
}

impl<T> Geomag for &T
where
    T: Model,
{
    fn at_location(self, geodetic: &GeodeticLocation) -> MagneticField {
        let geocentric = GeocentricLocation::from(geodetic);
        let mag = Calculator::new(self.deg(), self, &geocentric, geodetic);
        let xyz = mag.xyz();
        xyz.into()
    }
}
