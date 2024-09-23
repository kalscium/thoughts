use thoughts::log::Logger;

pub static LOGGER: Logger = Logger;

fn main() {
    // setup logging & panic hook
    let _ = log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Info));
    color_eyre::install().unwrap();
}
