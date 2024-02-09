# libgeomag
Rust library for calculating geomagnetic field models

## usage
```toml
[dependencies.geomag]
git = "https://github.com/SanmerDev/geomag.git"
package = "geomag"
```

## demo
```rust
use geomag::{DateTime, GeodeticLocation, Geomag, IGRF, WMM};

fn main() {
    let l = GeodeticLocation::new(102.0, 24.0, 1.9);
    let t = DateTime::new(2023, 11, 1, 0, 0, 0).unwrap();

    let wmm = WMM::new(t.decimal).unwrap();
    let igrf = IGRF::new(t.decimal).unwrap();

    let m = wmm.at_location(&l);
    println!("{:?}", m);

    let m = igrf.at_location(&l);
    println!("{:?}", m);
}
```

## References
- Chulliat, A. et al. (2020). The US/UK World Magnetic Model for 2020-2025 : Technical Report. [https://doi.org/10.25923/ytk1-yx35](https://doi.org/10.25923/ytk1-yx35)
- Alken, P., Thébault, E., Beggan, C.D., et al. (2021). International Geomagnetic Reference Field: the thirteenth generation. Earth Planets Space, 73(1), 49. [https://doi.org/10.1186/s40623-020-01288-x](https://doi.org/10.1186/s40623-020-01288-x)
