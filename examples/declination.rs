use time::{Date, Month};
use wmm::declination;

fn main() {
    let date = Date::from_calendar_date(2021, Month::September, 1).unwrap();
    let lat = 29.7363025;
    let lon = -93.8827939;
    let dec = declination(date, lat, lon).unwrap();

    println!(
        "Today's declination for coordinates {},{} is {}°",
        lat, lon, dec
    )
}
