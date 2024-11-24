extern crate log;

use clap::Parser;
// use log::Level;
use log::{info, LevelFilter}; // , SetLoggerError};
use syslog::{BasicLogger, Facility, Formatter3164};

mod config;
mod dhcp_client;
mod dhcp_server;
mod dns_client;
mod dns_server;
mod hnd_p2p;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Override the default configuration file
    #[arg(short, long, default_value = crate::config::CONFIG)]
    config: String,

    /// Check the configuration file
    #[arg(long)]
    check: bool,

    /// Specify an alternate log file
    #[arg(long, short, default_value = crate::config::LOGFILE)]
    logfile: String,

    /// Set the log level
    #[arg(long, default_value = crate::config::LOGLEVEL)]
    loglevel: String,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

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

    info!("{}", args.loglevel);
    info!("Hello, world!");
}
