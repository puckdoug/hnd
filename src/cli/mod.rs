use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Override the default configuration file
    #[arg(short, long, default_value = crate::config::CONFIG)]
    pub config: String,

    /// Check the configuration file
    #[arg(long)]
    pub check: bool,

    /// Specify an alternate log file
    #[arg(long, short, default_value = crate::config::LOGFILE)]
    pub logfile: String,

    /// Set the log level
    #[arg(long, default_value = crate::config::LOGLEVEL)]
    pub loglevel: String,

    #[arg(short, long)]
    pub verbose: bool,
}

pub fn parse() -> Args {
    Args::parse()
}
