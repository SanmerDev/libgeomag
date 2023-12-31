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
    pub d: f64,
    pub d_dot: f64,
    pub i: f64,
    pub i_dot: f64,
}
