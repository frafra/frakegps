extern crate time;

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

pub fn rmc(lat: f64, lon: f64) -> String {
    let lat_nmea = format!("{:08.3}", dec_to_dm(lat)*100.0);
    let lon_nmea = format!("{:09.3}", dec_to_dm(lon)*100.0);
    let ns = if lat.is_sign_negative() { 'S' } else { 'N' };
    let ew = if lon.is_sign_negative() { 'W' } else { 'E' };
    let now = time::now_utc();
    let date = now.strftime("%d%m%y").unwrap();
    let timestamp = now.strftime("%H%M%S").unwrap();
    let message = [
            "GPRMC",
            &timestamp.to_string(),
            "A", // A: valid, V: invalid
            &lat_nmea,
            &ns.to_string(),
            &lon_nmea,
            &ew.to_string(),
            "0", // speed over ground (knots)
            "0", // course over ground (degrees)
            &date.to_string(),
            "", // magnetic variation (degrees)
            "", // E/W (east/west)
        ].join(",");
    let checksum = checksum(&message);
    format!("${}*{}", message, checksum)
}
