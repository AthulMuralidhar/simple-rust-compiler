use log::LevelFilter;

pub fn init_logger(verbose: u32) {
    return env_logger::builder()
    .format_timestamp(None)
    .format_module_path(false)
    .filter_level(match verbose {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Warn,
        _ => LevelFilter::Trace,
    })
    .init();
}
