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

pub fn init() -> Result<(), SetLoggerError> {
    set_max_level(LevelFilter::Trace);
    set_logger(&LOGGER)
}
