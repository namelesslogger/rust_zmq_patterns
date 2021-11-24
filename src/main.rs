extern crate clap;
mod utils;
mod sockets;
use sockets::{publish, reply, request, subscribe, rout, deal};
use clap::{App, load_yaml};
use std::str::FromStr;
use std::thread;


fn generate_publisher_socket() {
    thread::spawn(|| {
        publish::run();
    });
}

fn generate_subscribe_socket() {
    thread::spawn(|| {
        subscribe::run();
    });
}

fn generate_reply_socket() {
    thread::spawn(|| {
        reply::run();
    });
}

fn generate_request_socket() {
    thread::spawn(|| {
        request::run();
    });
}

enum SupportedSockets {
    PUB,
    SUB,
    REP,
    REQ
}

impl FromStr for SupportedSockets {

    type Err = ();

    fn from_str(input: &str) -> Result<SupportedSockets, Self::Err> {
        match input {
            "PUB" => Ok(SupportedSockets::PUB),
            "SUB" => Ok(SupportedSockets::SUB),
            "REP" => Ok(SupportedSockets::REP),
            "REQ" => Ok(SupportedSockets::REQ),
            _      => Err(()),
        }
    }
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let _matches = App::from(yaml).get_matches();
    let pattern = load_yaml!("patterns/pubsub.yaml");

    for socket in pattern["sockets"].as_vec().unwrap() {
        let socket_description = socket.as_hash().unwrap().iter().next().unwrap();
        let socket_type = socket_description.0;
        //let socket_definition = socket_description.1;

        let socket_type = SupportedSockets::from_str(socket_type.as_str().unwrap()).expect("Unsupported socket type in schema definition, ignoring.");
        match socket_type {
            SupportedSockets::PUB => generate_publisher_socket(),
            SupportedSockets::SUB => generate_subscribe_socket(),
            SupportedSockets::REP => generate_reply_socket(),
            SupportedSockets::REQ => generate_request_socket(),
        }
    }
}
