use crate::num::{Degree, Float, Angle};

const A: f64 = 6378137.0;
const F: f64 = 1.0 / 298.257223563;

#[derive(Debug, Copy, Clone)]
pub struct GeodeticLocation {
    pub longitude: f64,
    pub latitude: f64,
    pub height: f64,
}

impl GeodeticLocation {
    pub fn new(longitude: Degree, latitude: Degree, height_m: f64) -> Self {
        let lat = if (90.0 - latitude.f()) < 1e-10 {
            (90.0 - 1e-6).deg()
        } else if (90.0 + latitude.f()) < 1e-10 {
            (-90.0 + 1e-6).deg()
        } else {
            latitude
        };

        GeodeticLocation {
            longitude: longitude.rad().f(),
            latitude: lat.rad().f(),
            height: height_m,
        }
    }
}

pub(crate) struct GeocentricLocation {
    pub longitude: f64,
    pub latitude: f64,
    pub radius: f64,
}

impl From<&GeodeticLocation> for GeocentricLocation {
    fn from(l: &GeodeticLocation) -> Self {
        let e_2 = F * (2.0 - F);
        let rc = A / (1.0 - e_2 * l.latitude.sin().powi(2)).sqrt();

        let p = (rc + l.height) * l.latitude.cos();
        let z = (rc * (1.0 - e_2) + l.height) * l.latitude.sin();

        let r = (p.powi(2) + z.powi(2)).sqrt();
        let lat = (z / r).asin();

        GeocentricLocation {
            longitude: l.longitude,
            latitude: lat,
            radius: r,
        }
    }
}
