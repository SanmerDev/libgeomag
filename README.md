# libgeomag
Rust library for calculating geomagnetic field models

## usage
```toml
libgeomag = { git = "https://github.com/SanmerDev/libgeomag.git", branch = "main" }
```

## demo
```rust
use libgeomag::{DateTime, GeodeticLocation, ModelExt, IGRF, WMM};

fn main() {
    let l = GeodeticLocation::new(102.0, 24.0, 1.9);
    let t = DateTime::new(2023, 11, 1, 0, 0, 0);

    let wmm = WMM::new(t.decimal).unwrap();
    let igrf = IGRF::new(t.decimal).unwrap();

    let m = wmm.single(l);
    println!("{:?}", m);

    let m = igrf.single(l);
    println!("{:?}", m);
}
```

## WMM
| Name     | Range              | Unit         |
|:---------:|:------------------:|:------------:|
| Latitude  | -90.00 to +90.00   | deg (WGS 84) |
| Longitude | -180.00 to +180.00 | deg (WGS 84) |
| Altitude  | -1.00 to 850.00    | km (WGS 84)  |
| Date      | 2020.00 to 2025.00 | -            |

## IGRF
| Name     | Range              | Unit         |
|:---------:|:------------------:|:------------:|
| Latitude  | -90.00 to +90.00   | deg (WGS 84) |
| Longitude | -180.00 to +180.00 | deg (WGS 84) |
| Altitude  | -1.00 to 600.00    | km (WGS 84)  |
| Date      | 1900.00 to 2025.00 | -            |

## MagneticField
| Field  | Name                | Unit   |
|:-----:|:--------------------:|:------:|
| x     | North Component      | nT     |
| x_dot | North SV             | nT/yr  |
| y     | East Component       | nT     |
| y_dot | East SV              | nT/yr  |
| z     | Vertical Component   | nT     |
| z_dot | Vertical SV          | nT/yr  |
| h     | Horizontal Intensity | nT     |
| h_dot | Horizontal SV        | nT/yr  |
| f     | Total Intensity      | nT     |
| f_dot | Total SV             | nT/yr  |
| d     | Declination          | rad    |
| d_dot | Declination SV       | rad/yr |
| i     | Inclination          | rad    |
| i_dot | Inclination SV       | rad/yr |

## References
- Chulliat, A. et al. (2020). The US/UK World Magnetic Model for 2020-2025 : Technical Report. [https://doi.org/10.25923/ytk1-yx35](https://doi.org/10.25923/ytk1-yx35)
- Alken, P., Thébault, E., Beggan, C.D., et al. (2021). International Geomagnetic Reference Field: the thirteenth generation. Earth Planets Space, 73(1), 49. [https://doi.org/10.1186/s40623-020-01288-x](https://doi.org/10.1186/s40623-020-01288-x)
