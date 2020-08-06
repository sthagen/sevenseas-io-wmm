#![no_std]

use libc::c_float;
use time::Date;

extern "C" {
    /**
     * Initialize the WMM. Needs calling only once.
     */
    fn wmm_init();

    /**
     * Get the date in WMM format
     *
     * @param year Year in 2 digit format of 21st centuary, i.e. 20 represents 2020
     * @param month Month, 1 to 12
     * @param day Date of month, 1 to 31
     * @return Date in WMM format
     * @note No checking of illegal dates is done
     */
    fn wmm_get_date(year: u8, month: u8, day: u8) -> c_float;

    /**
     * Get the magnetic variation at a point on the earth's surface
     *
     * @param glat Latitude in degrees and fractional degrees, negative west
     * @param glon Longitude in degrees and fractional degrees, negative west
     * @param time_years The date as returned from wmm_get_date
     * @param dec Pointer to float holding calculated magnetic variation (also known as declination). Negative is west.
     * @note The altitude used is the ellipsoid at the supplied latitude/longitude, not the earth's surface. This will
     *       give very small errors in some parts of the world comapred to sea level.
     */
    fn E0000(glat: c_float, glon: c_float, time_years: c_float, dec: *mut c_float);
}

static mut INITIALIZED: bool = false;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidDate,
}

pub fn declination(date: Date, lat: f32, lon: f32) -> Result<f32, Error> {
    unsafe {
        if !INITIALIZED {
            INITIALIZED = true;
            wmm_init();
        }
    }

    let year = date.year();
    if year < 2020 || year > 2025 {
        return Err(Error::InvalidDate);
    }

    let year = (year - 2000) as u8;
    unsafe {
        let date = wmm_get_date(year, date.month(), date.day());

        let mut dec: c_float = 0.0;
        let dec_ptr: *mut c_float = &mut dec;
        E0000(lat, lon, date, dec_ptr);
        Ok(dec)
    }
}

#[cfg(test)]
mod tests {
    use crate::declination;
    use crate::Error;
    use time::{date, Date};

    #[test]
    fn test_valid() -> Result<(), Error> {
        struct TestCase {
            date: Date,
            lat: f32,
            lon: f32,
            dec: f32
        }
        let test_cases = [
            TestCase {
                date: date!(2020 - 08 - 05),
                lat: 29.7363025,
                lon: -93.8827939,
                dec: 1.34724259
            },
            TestCase {
                date: date!(2020 - 08 - 05),
                lat: 43.34380925,
                lon: -4.3274906,
                dec: -0.532184601
            },
        ];

        for i in &test_cases {
            let target = declination(i.date, i.lat, i.lon)?;
            assert_eq!(target, i.dec);
        }

        Ok(())
    }

    #[test]
    fn test_invalid_date() {
        struct TestCase {
            date: Date,
            lat: f32,
            lon: f32,
        }
        let test_cases = [
            TestCase {
                date: date!(2019 - 12 - 31),
                lat: 29.7363025,
                lon: -93.8827939,
            },
            TestCase {
                date: date!(2026 - 01 - 01),
                lat: 29.7363025,
                lon: -93.8827939,
            },
        ];

        for i in &test_cases {
            let result = declination(i.date, i.lat, i.lon);
            let expected = Err(Error::InvalidDate);
            assert_eq!(expected, result);
        }
    }
}
