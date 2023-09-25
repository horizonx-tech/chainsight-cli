use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "test")]
/// Test your Chainsight's project.
pub struct TestOpts {}

#[cfg(test)]
pub mod runner {
    use std::panic;

    pub fn run_with(
        set_up: impl FnOnce() -> (),
        test: impl FnOnce() -> () + panic::UnwindSafe,
        tear_down: impl FnOnce() -> (),
    ) -> () {
        set_up();
        let result = panic::catch_unwind(|| test());
        tear_down();
        if let Err(err) = result {
            panic::resume_unwind(err);
        }
    }

    pub fn run_with_teardown(
        test: impl FnOnce() -> () + panic::UnwindSafe,
        teardown: impl FnOnce() -> (),
    ) -> () {
        let dummy_setup = || {};
        run_with(dummy_setup, test, teardown)
    }
}
