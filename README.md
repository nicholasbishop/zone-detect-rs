# zone-detect

This is a Rust version of the
[ZoneDetect](https://github.com/BertoldVdb/ZoneDetect) C library. The
initial conversion was done with c2rust, then manually cleaned up (it
no longer contains any unsafe code).

This crate can be used to look up the country and timezone of any
location on Earth.

## Running the example

    cargo run --example demo timezone21.bin 35.0715 -82.5216
