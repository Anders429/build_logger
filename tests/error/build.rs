fn main() {
    build_logger::init().expect("failed to initialize build_logger");

    log::error!("Hello, world!");
}
