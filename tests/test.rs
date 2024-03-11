use geomag::model::{IGRF, WMM};
use geomag::num::Angle;
use geomag::{DateTime, GeodeticLocation, Geomag};

fn get_decimal() -> f64 {
    let t = DateTime::new(2023, 11, 1, 0, 0, 0).unwrap();
    let decimal = t.decimal();
    assert_eq!(decimal, 2023.8328767123287);

    decimal
}

#[test]
fn wmm() {
    let l = GeodeticLocation::new(102.0.deg(), 24.0.deg(), 1900.0);
    let wmm = WMM::new(get_decimal()).unwrap();
    let m = wmm.at_location(&l);

    assert!(m.x - 37637.0 < 1.0);
    assert!(m.x_dot - 4.0 < 1.0);
    assert!(m.y - (-1100.0) < 1.0);
    assert!(m.y_dot - (-40.0) < 1.0);
    assert!(m.z - 28826.0 < 1.0);
    assert!(m.z_dot - 106.0 < 1.0);
    assert!(m.h - 37653.0 < 1.0);
    assert!(m.h_dot - 5.0 < 1.0);
    assert!(m.f - 47420.0 < 1.0);
    assert!(m.f_dot - 69.0 < 1.0);
    assert!(m.d.f() - (-0.02) < 0.01);
    assert!(m.d_dot.f() - (-0.001) < 0.001);
    assert!(m.i.f() - 0.6 < 0.1);
    assert!(m.i_dot.f() - 0.001 < 0.001);
}

#[test]
fn igrf() {
    let l = GeodeticLocation::new(102.0.deg(), 24.0.deg(), 1900.0);
    let igrf = IGRF::new(get_decimal()).unwrap();
    let m = igrf.at_location(&l);

    assert!(m.x - 37634.0 < 1.0);
    assert!(m.x_dot - 1.0 < 1.0);
    assert!(m.y - (-1103.0) < 1.0);
    assert!(m.y_dot - (-40.0) < 1.0);
    assert!(m.z - 28846.0 < 1.0);
    assert!(m.z_dot - 112.0 < 1.0);
    assert!(m.h - 37650.0 < 1.0);
    assert!(m.h_dot - 2.0 < 1.0);
    assert!(m.f - 47430.0 < 1.0);
    assert!(m.f_dot - 70.0 < 1.0);
    assert!(m.d.f() - (-0.02) < 0.01);
    assert!(m.d_dot.f() - (-0.001) < 0.001);
    assert!(m.i.f() - 0.6 < 0.1);
    assert!(m.i_dot.f() - 0.001 < 0.001);
}

#[test]
fn wmm_at_pole() {
    let l = GeodeticLocation::new(0.0.deg(), 90.0.deg(), 1900.0);
    let wmm = WMM::new(get_decimal()).unwrap();
    let m = wmm.at_location(&l);

    assert!(m.x - 1717.0 < 1.0);
    assert!(m.x_dot - (-27.0) < 1.0);
    assert!(m.y - 358.0 < 1.0);
    assert!(m.y_dot - 63.0 < 1.0);
    assert!(m.z - 56776.0 < 1.0);
    assert!(m.z_dot - 24.0 < 1.0);
    assert!(m.h - 1754.0 < 1.0);
    assert!(m.h_dot - (-14.0) < 1.0);
    assert!(m.f - 56803.0 < 1.0);
    assert!(m.f_dot - 23.0 < 1.0);
    assert!(m.d.f() - 0.2 < 0.1);
    assert!(m.d_dot.f() - 0.03 < 0.01);
    assert!(m.i.f() - 1.0 < 1.0);
    assert!(m.i_dot.f() - 0.0002 < 0.0001);
}

#[test]
fn igrf_at_pole() {
    let l = GeodeticLocation::new(0.0.deg(), 90.0.deg(), 1900.0);
    let igrf = IGRF::new(get_decimal()).unwrap();
    let m = igrf.at_location(&l);

    assert!(m.x - 1711.0 < 1.0);
    assert!(m.x_dot - (-24.0) < 1.0);
    assert!(m.y - 364.0 < 1.0);
    assert!(m.y_dot - 62.0 < 1.0);
    assert!(m.z - 56778.0 < 1.0);
    assert!(m.z_dot - 25.0 < 1.0);
    assert!(m.h - 1749.0 < 1.0);
    assert!(m.h_dot - (-10.0) < 1.0);
    assert!(m.f - 56805.0 < 1.0);
    assert!(m.f_dot - 24.0 < 1.0);
    assert!(m.d.f() - 0.2 < 0.1);
    assert!(m.d_dot.f() - 0.03 < 0.01);
    assert!(m.i.f() - 1.0 < 1.0);
    assert!(m.i_dot.f() - 0.0002 < 0.0001);
}
