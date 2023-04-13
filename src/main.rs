use geo::Point;
use time::{Date, Month};
use wkt::TryFromWkt;
use wmm::declination;

fn main() {
    let date = Date::from_calendar_date(2022, Month::December, 7).unwrap();
    let true_north = 1.5707963267948966; // assuming that true north is at 0 degrees

    let coords = vec![
        "POINT (5.250744062 51.794809988)",
        "POINT (5.250804877 51.794825014)",
        "POINT (5.25087085 51.794806756)",
        "POINT (5.25093729 51.794788589)",
        "POINT (5.251003619 51.794770417)",
        "POINT (5.251070246 51.79475163)",
        "POINT (5.251136548 51.794733338)",
        "POINT (5.251202792 51.794715031)",
        "POINT (5.251269206 51.794696656)",
        "POINT (5.251335485 51.79467775)",
        "POINT (5.251401806 51.794659382)",
        "POINT (5.25146818 51.794641061)",
    ];

    for coord in coords {
        let point: Point<f32> = Point::try_from_wkt_str(&coord).unwrap();
    
        let lon = point.x();
        let lat = point.y();

        let dec = declination(date, lat, lon).unwrap();
        let orientation = (lon - true_north - dec) / 2.0;

        println!(
            "Magnetic declination for coordinates {},{} is {:.2}°",
            lat, lon, dec
        );
        println!(
            "Orientation relative to true north for coordinates {},{} is {:.2} degrees",
            lat, lon, orientation
        );
    }
}
