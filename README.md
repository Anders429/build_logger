# build_logger

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Anders429/build_logger/ci.yml?branch=master)](https://github.com/Anders429/build_logger/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/build_logger)](https://crates.io/crates/build_logger)
[![docs.rs](https://docs.rs/build_logger/badge.svg)](https://docs.rs/build_logger)
[![License](https://img.shields.io/crates/l/build_logger)](#license)

A logger to be used in build scripts.

Allows logging through the [`log`](https://crates.io/crates/log) crate within a build script. Log messages are displayed as [Cargo warning](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning) messages.

## Usage

To use `build_logger`, initialize it by calling `init()`. After this, using the `log` crate's logging facade will display as warnings through Cargo.

``` rust
//! build.rs

fn main() {
    build_logger::init().expect("could not initialize build_logger");

    log::info!("Hello, world!");
}
```

## License
This project is licensed under either of

* Apache License, Version 2.0
([LICENSE-APACHE](https://github.com/Anders429/build_logger/blob/HEAD/LICENSE-APACHE) or
http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
([LICENSE-MIT](https://github.com/Anders429/build_logger/blob/HEAD/LICENSE-MIT) or
http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
