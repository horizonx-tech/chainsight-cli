use slog::{Logger, Level, Drain};

pub fn create_root_logger(verbose_level: i64) -> Logger {
    let log_level = match verbose_level {
        -3 => Level::Critical,
        -2 => Level::Error,
        -1 => Level::Warning,
        0 => Level::Info,
        1 => Level::Debug,
        x => {
            if x > 0 {
                Level::Trace
            } else {
                return Logger::root(slog::Discard, slog::o!());
            }
        }
    };

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog::LevelFilter::new(drain, log_level).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    // Logger::root(drain, slog::o!("version" => cli_version_str())) // if you want to display the cli version
    Logger::root(drain, slog::o!())
}
