use crate::logger::LogFormat;

// Example using the `log` crate.
// https://crates.io/crates/log

mod logger;

fn main() {
    setup_logger();

    log::info!("Start logging");
    log::warn!("Oh no, things might go wrong soon.");
    log::error!("Yeah, this is not good.");
    log::trace!("Something went wrong in `my service`.");
}

fn setup_logger() {
    use log::LevelFilter;
    // Setup logger and log level
    log::set_boxed_logger(Box::new(logger::Logger::custom(LogFormat::Json)))
        .expect("Could not setup logger");
    if cfg!(debug_assertions) {
        log::set_max_level(LevelFilter::Trace);
    } else {
        log::set_max_level(LevelFilter::Info);
    }
}
