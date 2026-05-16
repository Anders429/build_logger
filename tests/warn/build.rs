fn main() {
    build_logger::init().expect("failed to initialize build_logger");

    log::warn!("Hello, world!");
}
