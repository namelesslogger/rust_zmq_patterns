extern crate clap;
mod request;
mod utils;
use request::{ReqClient, RepClient};
use clap::{Arg, App};

fn main() {
    let matches = App::new("Zmq Test bed")
        .version("0.1")
        .author("Shane M <johnshanie@protonmail.com>")
        .about("I do things")
        .arg(Arg::with_name("server")
            .short("s")
            .long("server")
            .help("Sets process to act as server or client"))
        .get_matches();
    let server = matches.occurrences_of("server") > 0 ;

    if server {
        println!("Starting server process");
        let rep_client: RepClient = RepClient::new();
        rep_client.reply("I got a Messsage!!! Heres the response");
    } else {
        println!("Starting client process");
        let req_client: ReqClient = ReqClient::new();
        req_client.request("Ohhhh a request!!!");
    }
}
