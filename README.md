# Description

FrakeGPS simulates a simple GPS receiver which emits NMEA codes. The location is selected clicking on a map, using [web-view](https://github.com/Boscop/web-view) and [Leaflet](http://leafletjs.com/).

![frakegps-gpsmon](https://user-images.githubusercontent.com/4068/58375414-ba3b3900-7f52-11e9-88bb-c6db1299eff0.png)

# Build

## Dependencies

- cargo
- gpsd
- webkit2gtk (devel)

### Fedora

```
dnf install cargo gpsd webkit2gtk3-devel
```

## Compile

```
cargo build
```

# Usage

## Usage with gpsd

```
./target/debug/frakegps |& gpsd -n -N /dev/stdin
```

## Usage with geoclue

```
./target/debug/frakegps |& nc -kl 10110
avahi-publish -s "FrakeGPS for $(hostname)"  _nmea-0183._tcp 10110 "FrakeGPS service"
```

Note: Unreliable results with avahi.
