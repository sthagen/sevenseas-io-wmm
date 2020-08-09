# wmm

[![ci](https://github.com/sevenseas-io/wmm/workflows/ci/badge.svg)](https://github.com/sevenseas-io/wmm/actions?query=workflow%3Aci)
[![license](https://img.shields.io/badge/license-MIT%20or%20Apache--2-brightgreen)](https://github.com/sevenseas-io/wmm#license)
[![version](https://img.shields.io/crates/v/wmm.svg)](https://crates.io/crates/wmm)
[![docs](https://docs.rs/wmm/badge.svg)](https://docs.rs/wmm/)

Low footprint `#[no_std]` World Magnetic Model (WMM) library used to calculate the magnetic declination at sea level.

It's important to note that the current model is valid from 2020 until 2025.

## Example

```
use time::OffsetDateTime;
use wmm::declination;

fn main() {
    let date = OffsetDateTime::now_utc().date();
    let lat = 29.7363025;
    let lon = -93.8827939;
    let dec = declination(date, lat, lon).unwrap();

    println!(
        "Today's declination for coordinates {},{} is {}°",
        lat, lon, dec
    )
}
```

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.35 and up. It *might* compile with older versions but that may change in any new patch release.

## Credits

The C code this library refences originates from [WMM_Tiny](https://github.com/miniwinwm/WMM_Tiny).

The [WMM](https://www.ngdc.noaa.gov/geomag/WMM/) is a NOAA effort which is part of the US Government.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
