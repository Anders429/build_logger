//! A logger to be used in build scripts.
//!
//! Allows logging through the [`log`] crate within a build script. Log messages are displayed as
//! [Cargo warning](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning)
//! messages.
//!
//! ## Usage
//! To use `build_logger`, initialize it by calling [`init()`]. After this, using the [`log`]
//! crate's logging facade will display as warnings through Cargo.
//!
//! ```
//! //! build.rs
//!
//! fn main() {
//!     build_logger::init().expect("could not initialize build_logger");
//!
//!     log::info!("Hello, world!");
//! }
//! ```

use log::{LevelFilter, Log, Metadata, Record, SetLoggerError, set_logger, set_max_level};

#[derive(Debug)]
struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let level = record.level();
        let target = record.target();
        let args = record.args();
        println!("cargo::warning={level}:{target} -- {args}");
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

/// Initialize the logger.
///
/// After calling this function, log messages created with the [`log`] crate will be displayed as
/// [Cargo warning](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargo-warning)
/// messages.
///
/// ```
/// //! build.rs
///
/// fn main() {
///     build_logger::init().expect("could not initialize build_logger");
///
///     log::info!("Hello, world!");
/// }
/// ```
pub fn init() -> Result<(), SetLoggerError> {
    set_max_level(LevelFilter::Trace);
    set_logger(&LOGGER)
}
