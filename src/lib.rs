#![no_std]

use libc::c_float;

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

pub fn declination(year: u8, month: u8, day: u8, lat: f32, lon: f32) -> f32 {
    unsafe {
        if !INITIALIZED {
            INITIALIZED = true;
            wmm_init();
        }
    }

    unsafe {
        let date = wmm_get_date(year, month, day);

        let mut dec: c_float = 0.0;
        let dec_ptr: *mut c_float = &mut dec;
        E0000(lat, lon, date, dec_ptr);
        dec
    }
}

#[cfg(test)]
mod tests {
    use crate::declination;

    #[test]
    fn test() {
        let target = declination(20, 8, 5, 29.7363025, -93.8827939);
        assert_eq!(target, 1.34724259);
    }
}
