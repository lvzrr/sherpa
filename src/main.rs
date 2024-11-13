mod filehandler;
mod server;
use filehandler::{get_file_contents, lookup, Database};
use server::Server;
use std::env;
use std::io::{stdin, stdout, Write};
use std::process::exit;
use std::str::FromStr;
mod client;
use client::Client;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/*
 * TODO implement asyc module and "setup" server-client interaction (improve frontend if possible)
 */

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"-h".to_string()) {
        let mut host = Server::init(8888);
        host.run();
    } else {
        let mut cli: Client = Client::init(&SocketAddr::new(
            IpAddr::V4(Ipv4Addr::from_str("192.168.1.58").unwrap()),
            8888,
        ));
        cli.run();
    }
}
