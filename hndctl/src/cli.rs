use clap::Parser;
use config;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Override the default configuration file
    #[arg(short, long, default_value = config::CONFIG)]
    pub config: String,

    /// Check the configuration file
    #[arg(long)]
    pub check: bool,

    #[arg(short, long)]
    pub verbose: bool,
}

pub fn parse() -> Args {
    Args::parse()
}
