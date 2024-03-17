# zone-detect

[![crates.io](https://img.shields.io/crates/v/zone-detect.svg)](https://crates.io/crates/zone-detect)
[![Documentation](https://docs.rs/zone-detect/badge.svg)](https://docs.rs/zone-detect)

This is a Rust version of the
[ZoneDetect](https://github.com/BertoldVdb/ZoneDetect) C library. The
initial conversion was done with c2rust, then manually cleaned up (it
no longer contains any unsafe code).

This crate can be used to look up the country and timezone of any
location on Earth.

## Running the example

```
$ cargo run --example demo data/timezone21.bin 35.0715 -82.5216
zone 0: ZoneMatch {
    kind: InZone,
    zone: Zone {
        polygon_id: 1458,
        meta_id: 3199,
        fields: {
            "CountryAlpha2": "US",
            "CountryName": "United States",
            "TimezoneIdPrefix": "America/",
            "TimezoneId": "New_York",
        },
    },
}
```

## Data source

The database containing the location and timezone data is in
`data/timezone21.bin`. It can be updated as follows:

```
git clone https://github.com/BertoldVdb/ZoneDetect
cd ZoneDetect/database/builder
./makedb.sh
cp out_v1/timezone21.bin zone-detect-rs/data/timezone21.bin
```
    
## Testing

There's a slow test that generates random values and compares the output
between ZoneDetect and zone-detect-rs.

```
# Make sure the demo is built first; just run `make` in the ZoneDetect repo.

ZONEDETECT_DEMO=../ZoneDetect/demo cargo test -- --ignored
```
