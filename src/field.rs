use crate::num::Rad;

#[derive(Debug, Copy, Clone)]
pub struct MagneticField {
    pub x: f64,
    pub x_dot: f64,
    pub y: f64,
    pub y_dot: f64,
    pub z: f64,
    pub z_dot: f64,
    pub h: f64,
    pub h_dot: f64,
    pub f: f64,
    pub f_dot: f64,
    pub d: Rad,
    pub d_dot: Rad,
    pub i: Rad,
    pub i_dot: Rad,
}
