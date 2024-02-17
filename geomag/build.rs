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

fn parse_wmm<P: AsRef<Path>>(p: P) -> Vec<Vec<f64>> {
    let constant = fs::read_to_string(p).unwrap();
    constant
        .lines()
        .map(|s| {
            let values_str = s.split_whitespace().collect::<Vec<&str>>();
            parse_str(&values_str[2..])
        })
        .collect()
}

fn parse_igrf<P: AsRef<Path>>(p: P) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let constant = fs::read_to_string(p).unwrap();
    let constant: Vec<Vec<&str>> = constant
        .lines()
        .map(|s| s.split_whitespace().collect())
        .collect();

    let mut values_g = Vec::new();
    let mut values_h = Vec::new();

    for values_str in constant {
        match values_str[0] {
            "g" => {
                let values = &values_str[3..];
                values_g.push(parse_str(values))
            }
            "h" => {
                let values = &values_str[3..];
                values_h.push(parse_str(values));

                // Keep the same index as 'values_g', and zero-pad empty data
                if values_str[1] == values_str[2] {
                    values_h.push(vec![0_f64; values.len()])
                }
            }
            _ => {}
        }
    }

    // Move last zero-pad data to first, h[0, 0]
    match values_h.pop() {
        None => {}
        Some(v) => values_h.insert(0, v),
    }

    (values_g, values_h)
}

fn main() {
    let data_dir = Path::new("data");

    let wmm = parse_wmm(data_dir.join("WMM.COF"));
    vec_to_out(wmm, "WMM_COF");

    let (igrf_g, igrf_h) = parse_igrf(data_dir.join("IGRF.COF"));
    vec_to_out(igrf_g, "IGRF_COF_G");
    vec_to_out(igrf_h, "IGRF_COF_H");
}
