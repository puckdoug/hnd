extern crate log;

use crate::cli::Args;

use log::{info, LevelFilter}; // , SetLoggerError};
use syslog::{BasicLogger, Facility, Formatter3164};

pub fn setup(args: &Args) {
    let log_level = match args.loglevel.as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        _ => LevelFilter::Info,
    };

    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "hnd".into(),
        pid: 0,
    };

    let logger = match syslog::unix(formatter) {
        Err(e) => {
            println!("impossible to connect to syslog: {:?}", e);
            return;
        }
        Ok(logger) => logger,
    };
    match log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(log_level))
    {
        Ok(()) => info!("Ok"),
        Err(e) => {
            eprintln!("Impossible to get logger for Syslog: {:?}", e);
            std::process::exit(-1);
        }
    }
}
