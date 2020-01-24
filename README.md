# zone-detect

This is a simple Rust library that wraps the
[ZoneDetect](https://github.com/BertoldVdb/ZoneDetect) C library.

## Submodule

The [ZoneDetect](https://github.com/BertoldVdb/ZoneDetect) repo, which
provides the actual implementation, is pulled in as a submodule. Make
sure to run `git submodule update --init` after cloning this repo.

## Running the example

    cargo run --example demo timezone21.bin -- 35.0715 -82.5216

Note the `--`, that's needed if the latitude or longitude is negative.
