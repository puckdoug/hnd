// extern crate log;

use log::info;

mod cli;
mod config;
// mod dhcp_client;
// mod dhcp_server;
// mod dns_client;
// mod dns_server;
// mod hnd_p2p;
mod logs;

fn main() {
    let args = cli::parse();
    logs::setup(&args);

    info!("Hello, world!");
}
