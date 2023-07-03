pub struct EnvironmentImpl {
    logger: Option<slog::Logger>,
}

impl EnvironmentImpl {
    pub fn new() -> Self {
        Self { logger: None }
    }

    pub fn with_logger(mut self, logger: slog::Logger) -> Self {
        self.logger = Some(logger);
        self
    }

    pub fn get_logger(&self) -> &slog::Logger {
        self.logger
            .as_ref()
            .expect("Log was not setup, but is being used.")
    }
}
