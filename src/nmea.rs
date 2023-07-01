use time::format_description::FormatItem;
use time::macros::format_description;
use time::OffsetDateTime;

fn dec_to_dm(dec: f64) -> f64 {
    let deg = dec.abs().floor();
    let min = (dec - deg) * 60.0 / 100.0;
    deg + min
}

fn checksum(message: &String) -> String {
    let mut checksum = 0;
    for character in message.bytes() {
        checksum ^= character;
    }
    format!("{:02X}", checksum)
}

const TIMESTAMP_FORMAT: &[FormatItem<'_>] =
    format_description!("[hour][minute][second].[subsecond digits:3]");

pub fn gga(lat: f64, lon: f64, altitude: f64) -> String {
    let lat_nmea = format!("{:08.3}", dec_to_dm(lat) * 100.0);
    let lon_nmea = format!("{:09.3}", dec_to_dm(lon) * 100.0);
    let ns = if lat.is_sign_negative() { 'S' } else { 'N' };
    let ew = if lon.is_sign_negative() { 'W' } else { 'E' };
    let now = OffsetDateTime::now_utc();
    let timestamp = now.format(&TIMESTAMP_FORMAT).unwrap();
    let message = [
        "GPGGA",
        &timestamp,
        &lat_nmea,
        &ns.to_string(),
        &lon_nmea,
        &ew.to_string(),
        "1", // Fix valid
        "",  // Satellites used
        "",  // HDOP
        &altitude.to_string(),
        "M",    // meters
        "0",    // Geoid separation
        "M",    // meters
        "",     // DGPS
        "0000", // DGPS station ID
    ]
    .join(",");
    let checksum = checksum(&message);
    format!("${}*{}", message, checksum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gga_test() {
        let result = gga(63.43062267277069, 10.39487421512604, 0.0);
        let expected = ",6325.837,N,01023.692,E,1,,,0,M,0,M,,0000*";
        assert_eq!(&result[17..result.len() - 2], expected);
    }
}
