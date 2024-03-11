use crate::num::Radian;

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
    pub d: Radian,
    pub d_dot: Radian,
    pub i: Radian,
    pub i_dot: Radian,
}
