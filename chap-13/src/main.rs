use std::fmt::Display;

pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: impl Display);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: impl Display) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

fn do_things(logger: &impl Logger) {
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}

// TODO: Define and implement `VerbosityFilter`.
struct VerbosityFilter<T : Logger> {
    max_verbosity: u8,
    inner: T,
}

impl<T : Logger> Logger for VerbosityFilter<T> {
    fn log(&self, verbosity: u8, message: impl Display) {
        if (self.max_verbosity <= verbosity) {
            self.inner.log(verbosity, message)
        }
    }
}


fn main() {
    let l = VerbosityFilter { max_verbosity: 3, inner: StderrLogger };
    do_things(&l);
}