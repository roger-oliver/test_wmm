use time::{OffsetDateTime, Date};
use wmm::declination;

fn main() {
    let date = Date::from_calendar_date(2023, time::Month::February, 9).unwrap();  // OffsetDateTime::now_utc().date();
    let lat = 5.251070246;
    let lon = 51.79475163;
    let dec = declination(date, lat, lon).unwrap();

    println!(
        "Today's declination for coordinates {},{} is {}°",
        lat, lon, dec
    );

    let true_north = 0.0; // assuming that true north is at 0 degrees
    let orientation = (lon - true_north - dec.to_degrees()).to_degrees() / 360.0;

    println!("Orientation relative to true north: {:.2} degrees", orientation);

}