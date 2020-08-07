# World Magnetic Model

![build](https://img.shields.io/github/workflow/status/sevenseas-io/wmm/Continuous%20Integration)
![crates.io](https://img.shields.io/crates/v/wmm.svg)

Low footprint *no_std* WMM library used to calculate the magnetic declination at sea level.

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

## Credits

The C code this lirary refences originates from [WMM_Tiny](https://github.com/miniwinwm/WMM_Tiny).

The [WMM](https://www.ngdc.noaa.gov/geomag/WMM/) is a NOAA effort which is part of the US Government.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.