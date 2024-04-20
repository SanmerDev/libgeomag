use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use std::{env, fs};

#[inline]
fn parse_str(values: &[&str]) -> Vec<f64> {
    values.iter().map(|s| f64::from_str(s).unwrap()).collect()
}

#[inline]
fn vec_to_out<P: AsRef<Path>>(v: Vec<Vec<f64>>, file: P) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    File::create(out_dir.join(file))
        .unwrap()
        .write_all(format!("{:?}", v).as_bytes())
        .unwrap();
}

#[cfg(feature = "wmm")]
fn parse_wmm<P: AsRef<Path>>(p: P) {
    let content = fs::read_to_string(p).unwrap();
    let constant = content
        .lines()
        .map(|s| {
            let values_str = s.split_whitespace().collect::<Vec<&str>>();
            parse_str(&values_str[2..])
        })
        .collect();

    vec_to_out(constant, "WMM_COF");
}

#[cfg(not(feature = "wmm"))]
fn parse_wmm<P: AsRef<Path>>(_p: P) {}

#[cfg(feature = "igrf")]
fn parse_igrf<P: AsRef<Path>>(p: P) {
    let content = fs::read_to_string(p).unwrap();
    let constant: Vec<Vec<&str>> = content
        .lines()
        .map(|s| s.split_whitespace().collect())
        .collect();

    let mut constant_g = Vec::new();
    let mut constant_h = Vec::new();

    for values_str in constant {
        match values_str[0] {
            "g" => {
                let values = &values_str[3..];
                constant_g.push(parse_str(values));

                if values_str[2] == "0" {
                    constant_h.push(vec![0_f64; values.len()])
                }
            }
            "h" => {
                let values = &values_str[3..];
                constant_h.push(parse_str(values));
            }
            _ => {}
        }
    }

    vec_to_out(constant_g, "IGRF_COF_G");
    vec_to_out(constant_h, "IGRF_COF_H");
}

#[cfg(not(feature = "igrf"))]
fn parse_igrf<P: AsRef<Path>>(_p: P) {}

fn main() {
    let data_dir = Path::new("data");
    parse_wmm(data_dir.join("WMM.COF"));
    parse_igrf(data_dir.join("IGRF.COF"));
}
