extern crate chrono;
use chrono::prelude::*;

fn dec_to_dm(dec: f64) -> f64 {
    let deg = dec.abs().floor();
    let min = (dec-deg)*60.0/100.0;
    deg+min
}

fn checksum(message: &String) -> String {
    let mut checksum = 0;
    for character in message.bytes() {
        checksum ^= character;
    }
    format!("{:02X}", checksum)
}

pub fn gga(lat: f64, lon: f64, altitude: f64) -> String {
    let lat_nmea = format!("{:08.3}", dec_to_dm(lat)*100.0);
    let lon_nmea = format!("{:09.3}", dec_to_dm(lon)*100.0);
    let ns = if lat.is_sign_negative() { 'S' } else { 'N' };
    let ew = if lon.is_sign_negative() { 'W' } else { 'E' };
    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.format("%H%M%S.%3f");
    let message = [
            "GPGGA",
            &timestamp.to_string(),
            &lat_nmea,
            &ns.to_string(),
            &lon_nmea,
            &ew.to_string(),
            "1", // Fix valid
            "",  // Satellites used
            "",  // HDOP
            &altitude.to_string(),
            "M", // meters
            "0", // Geoid separation
            "M", // meters
            "",  // DGPS
            "0000", // DGPS station ID
        ].join(",");
    let checksum = checksum(&message);
    format!("${}*{}", message, checksum)
}
