[![Build Status](https://travis-ci.com/frafra/frakegps.svg?branch=master)](https://travis-ci.com/frafra/frakegps)

# Description

FrakeGPS simulates a simple GPS receiver which emits NMEA codes. The location is selected clicking on a map, using [web-view](https://github.com/Boscop/web-view) and [Leaflet](http://leafletjs.com/).

![frakegps-gpsmon](https://user-images.githubusercontent.com/4068/58375414-ba3b3900-7f52-11e9-88bb-c6db1299eff0.png)

# Build

## Dependencies

- cargo
- webkit2gtk (devel)

### Fedora

```
sudo dnf install cargo webkit2gtk3-devel
```

### Ubuntu

```
sudo apt-get install cargo libwebkit2gtk-4.0-dev
```

## Compile

```
cargo build
```

# Usage

## Usage with gpsd

```
sudo systemctl stop gpsd.socket
cargo run -q | gpsd -bnN /dev/stdin
```

## Usage with geoclue

```
cargo run -q | nc -vkl 10110
avahi-publish -s "FrakeGPS for $(hostname)" _nmea-0183._tcp 10110
```

# Additional resources

- [NMEA reference](https://www.sparkfun.com/datasheets/GPS/NMEA%20Reference%20Manual-Rev2.1-Dec07.pdf)
- [gclue-nmea-source.c](https://gitlab.freedesktop.org/geoclue/geoclue/blob/master/src/gclue-nmea-source.c) from geoclue
- [avahi.rs](https://github.com/zeenix/gps-share/blob/master/src/avahi.rs) from gps-share
