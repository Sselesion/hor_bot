use log::*;
use std::env::{set_var, var};

// #[macro_use]
// extern crate log;
// extern crate pretty_env_logger;

/// Start env_logger
pub fn init_logging(level: u8) {
    // if RUST_BACKTRACE is set, ignore the arg given and set `trace` no matter what
    let mut overridden = false;
    let verbosity = if var("RUST_LOG").unwrap_or_else(|_| "0".into()) == "1" {
        overridden = true;
        "trace"
    } else {
        match level {
            0 => "error",
            1 => "warn",
            2 => "info",
            3 => "debug",
            _ => "trace",
        }
    };
    set_var("RUST_LOG", verbosity);

    pretty_env_logger::init();

    if overridden {
        warn!("RUST_BACKTRACE is set, overriding user verbosity level");
    } else if verbosity == "trace" {
        set_var("RUST_BACKTRACE", "1");
        trace!("RUST_BACKTRACE has been set");
    };
    info!(
        "Set verbosity to {}",
        var("RUST_LOG").expect("Should set RUST_LOG environment variable")
    );
}
