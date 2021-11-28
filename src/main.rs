extern crate clap;
mod utils;
mod sockets;
use sockets::{publish, reply, request, subscribe};
use clap::{App, load_yaml};
use std::str::FromStr;
use std::thread;


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
    let mut children: Vec<std::thread::JoinHandle<()>> = Vec::new();

    for socket in pattern["sockets"].as_vec().unwrap() {
        let socket_description = socket.as_hash().unwrap().iter().next().unwrap();
        let socket_type = socket_description.0;
        //let socket_definition = socket_description.1;

        let socket_type = SupportedSockets::from_str(socket_type.as_str().unwrap()).expect("Unsupported socket type in schema definition, ignoring.");
        let thread_spawned = match socket_type {
            SupportedSockets::PUB => {
                thread::spawn(|| {
                    publish::run();
                })
            },
            SupportedSockets::SUB =>  {
                thread::spawn(|| {
                    subscribe::run();
                })
            },
            SupportedSockets::REP => {
                thread::spawn(|| {
                    reply::run();
                })
            },
            SupportedSockets::REQ => {
                thread::spawn(|| {
                    request::run();
                })
            }
        };
        children.push(thread_spawned);
    }

    for child in children {
        let _ = child.join();
    }
}
