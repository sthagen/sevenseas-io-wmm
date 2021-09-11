#![no_std]

use cty::c_float;
use spin::Mutex;
use time::Date;

extern "C" {
    fn wmm_init();
    fn wmm_get_date(year: u8, month: u8, day: u8) -> c_float;
    fn E0000(glat: c_float, glon: c_float, time_years: c_float, dec: *mut c_float);
}

/// WMM Error
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    /// Invalid date
    ///
    /// The current WMM only supports dates within the 2020 to 2025 time range
    InvalidDate,

    /// Invalid coordinates
    ///
    /// Valid entries are:
    /// * Latitude -90.00 to +90.00 degrees  
    /// * Longitude -180.00 to +180.00 degrees  
    InvalidCoordinates,
}

/// Returns the magnetic declination for a given date and location
///
/// # Arguments
///
/// * `date` - Date within the 2020 to 2025 time range
/// * `lat` - Latitude: -90.00 to +90.00 degrees
/// * `lon` - Longitude: -180.00 to +180.00 degrees
///
/// # Examples
///
/// ```
/// use time::{Date, Month};
/// use wmm::declination;
/// let date = Date::from_calendar_date(2021, Month::September, 1).unwrap();
/// let lat = 29.7363025;
/// let lon = -93.8827939;
/// let dec = declination(date, lat, lon).unwrap();
/// ```
pub fn declination(date: Date, lat: f32, lon: f32) -> Result<f32, Error> {
    static INITIALIZED: Mutex<bool> = Mutex::new(false);
    {
        let mut initialized = INITIALIZED.lock();
        if !*initialized {
            unsafe {
                wmm_init();
            }
            *initialized = true;
        }
    }

    let year = date.year();
    if !(2020..=2024).contains(&year) {
        return Err(Error::InvalidDate);
    }

    if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
        return Err(Error::InvalidCoordinates);
    }

    let year = (year - 2000) as u8;
    unsafe {
        let date = wmm_get_date(year, date.month() as u8, date.day());

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
    use time::{Date, Month};

    #[test]
    fn test_valid() -> Result<(), Error> {
        struct TestCase {
            date: Date,
            lat: f32,
            lon: f32,
            dec: f32,
        }
        let test_cases = [
            TestCase {
                date: Date::from_calendar_date(2020, Month::August, 05).unwrap(),
                lat: 29.7363025,
                lon: -93.8827939,
                dec: 1.34724259,
            },
            TestCase {
                date: Date::from_calendar_date(2020, Month::August, 05).unwrap(),
                lat: 43.34380925,
                lon: -4.3274906,
                dec: -0.532184601,
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
                date: Date::from_calendar_date(2019, Month::December, 31).unwrap(),
                lat: 29.7363025,
                lon: -93.8827939,
            },
            TestCase {
                date: Date::from_calendar_date(2026, Month::January, 01).unwrap(),
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

    #[test]
    fn test_invalid_coordinates() {
        struct TestCase {
            date: Date,
            lat: f32,
            lon: f32,
        }
        let test_cases = [
            TestCase {
                date: Date::from_calendar_date(2020, Month::August, 07).unwrap(),
                lat: 90.00001,
                lon: -93.8827939,
            },
            TestCase {
                date: Date::from_calendar_date(2020, Month::August, 07).unwrap(),
                lat: -90.00001,
                lon: -93.8827939,
            },
            TestCase {
                date: Date::from_calendar_date(2020, Month::August, 07).unwrap(),
                lat: 29.7363025,
                lon: 180.00001,
            },
            TestCase {
                date: Date::from_calendar_date(2020, Month::August, 07).unwrap(),
                lat: 29.7363025,
                lon: -180.00001,
            },
        ];

        for i in &test_cases {
            let result = declination(i.date, i.lat, i.lon);
            let expected = Err(Error::InvalidCoordinates);
            assert_eq!(expected, result);
        }
    }
}
