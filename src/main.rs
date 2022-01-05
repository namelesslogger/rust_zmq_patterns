extern crate clap;
extern crate yaml_rust;
mod utils;
mod sockets;
use sockets::{publish, reply, request, subscribe, rout, deal};
use clap::{App, load_yaml};
use std::fs::File;
use std::str::FromStr;
use std::thread;
use yaml_rust::{YamlLoader};
use std::io::Read;


enum SupportedSockets {
    PUB,
    SUB,
    REP,
    REQ,
    DEAL,
    ROUT,
}

impl FromStr for SupportedSockets {

    type Err = ();

    fn from_str(input: &str) -> Result<SupportedSockets, Self::Err> {
        match input {
            "PUB" => Ok(SupportedSockets::PUB),
            "SUB" => Ok(SupportedSockets::SUB),
            "REP" => Ok(SupportedSockets::REP),
            "REQ" => Ok(SupportedSockets::REQ),            
            "DEAL" => Ok(SupportedSockets::DEAL),
            "ROUT" => Ok(SupportedSockets::ROUT),
            _      => Err(()),
        }
    }
}

struct SocketConfig {
    duplication: i64,
    host: String,
    port: i64,
    protocol: String,
    topics: String,
    socket_type: String
}

impl SocketConfig {
    fn new(socket_definition: yaml_rust::Yaml) -> SocketConfig {
        SocketConfig {
            duplication: socket_definition["duplication"].as_i64().expect("Invalid Integer value, should fit into i64 datatype"),
            host: socket_definition["host"].as_str().unwrap().to_string(),
            port: socket_definition["port"].as_i64().expect("Invalid Integer value, should fit into i64 datatype"),
            protocol: socket_definition["protocol"].as_str().unwrap().to_string(),
            topics: socket_definition["topics"].as_str().unwrap().to_string(),
            socket_type: socket_definition["socket_type"].as_str().unwrap().to_string()
        }
    }
}

fn load_file(file: &str) -> Vec<yaml_rust::Yaml>{
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    YamlLoader::load_from_str(&contents).unwrap()
}


fn run(config_file: &str) {
    let config_file_path: String = format!("src/patterns/{}.yaml", config_file);
    let pattern = load_file(&config_file_path)
                    .into_iter()
                    .next()
                    .expect("No config in file?");
    
    let mut children: Vec<std::thread::JoinHandle<()>> = Vec::new();

    for socket in pattern["sockets"].as_vec().unwrap() {
        let socket_description = socket.as_hash().unwrap().iter().next().unwrap();
        let socket_definition = socket_description.1;
        let duplication: i64 = socket_definition["duplication"].as_i64().expect("Invalid Integer value, should fit into i64 datatype");

        for _ in 0..duplication {
            let socket_type = SupportedSockets::from_str(socket_definition["socket_type"].as_str().unwrap()).expect("Unsupported socket type in schema definition, ignoring.");
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
                },
                SupportedSockets::DEAL => {
                    thread::spawn(|| {
                        deal::run();
                    })
                },
                SupportedSockets::ROUT => {
                    thread::spawn(|| {
                        rout::run();
                    })
                }
            };
            children.push(thread_spawned);
        }
    }

    for child in children {
        let _ = child.join();
    }
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    if let Some(c) = matches.value_of("config") {
        run(&c)
    } else {
        println!("missing value for config")
    }
}
