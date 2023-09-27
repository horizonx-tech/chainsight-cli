use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "test")]
/// Test your Chainsight's project.
pub struct TestOpts {}

#[cfg(test)]
pub mod tests {
    use crate::lib::environment::EnvironmentImpl;
    use crate::lib::logger::create_root_logger;

    pub fn run(
        setup: impl FnOnce() -> (),
        test: impl FnOnce() -> () + std::panic::UnwindSafe,
        teardown: impl FnOnce() -> (),
    ) {
        use std::panic;

        setup();
        let result = panic::catch_unwind(|| test());
        teardown();
        assert!(result.is_ok())
    }
    pub fn run_with_teardown(
        test: impl FnOnce() -> () + std::panic::UnwindSafe,
        teardown: impl FnOnce() -> (),
    ) {
        let dumy_setup = || {};
        run(dumy_setup, test, teardown)
    }
    pub fn test_env() -> EnvironmentImpl {
        EnvironmentImpl::new().with_logger(create_root_logger(1))
    }
}
