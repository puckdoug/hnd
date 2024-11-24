use clap::Parser;

mod config;
mod dhcp_client;
mod dhcp_server;
mod dns_client;
mod dns_server;
mod hnd_p2p;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, default_value = "/etc/hnd.toml")]
    config: String,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let _args = Args::parse();
    println!("Hello, world!");
}
