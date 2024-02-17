use num_traits::Float;

use crate::util::MathExt;

const A: f64 = 6378137.0;
const F: f64 = 1.0 / 298.257223563;

pub struct GeodeticLocation {
    pub longitude: f64,
    pub latitude: f64,
    pub height: f64,
}

impl GeodeticLocation {
    pub fn new(longitude: f64, latitude: f64, height: f64) -> Self {
        let lat = if (90.0 - latitude) < 1e-10 {
            90.0 - 1e-6
        } else if (90.0 + latitude) < 1e-10 {
            -90.0 + 1e-6
        } else {
            latitude
        };

        GeodeticLocation {
            longitude: longitude.to_rad(),
            latitude: lat.to_rad(),
            height: height * 1000.0,
        }
    }
}

impl From<&GeodeticLocation> for GeodeticLocation {
    fn from(l: &GeodeticLocation) -> Self {
        GeodeticLocation {
            longitude: l.longitude,
            latitude: l.latitude,
            height: l.height,
        }
    }
}

pub(crate) struct GeocentricLocation {
    pub geodetic: GeodeticLocation,
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

        Self {
            geodetic: l.into(),
            longitude: l.longitude,
            latitude: lat,
            radius: r,
        }
    }
}
