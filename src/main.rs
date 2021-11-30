extern crate clap;
extern crate yaml_rust;
mod utils;
mod sockets;
use sockets::{publish, reply, request, subscribe};
use clap::{App, load_yaml};
use std::fs::File;
use std::str::FromStr;
use std::thread;
use yaml_rust::YamlLoader;
use std::io::Read;


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

fn load_file(file: &str) -> Vec<yaml_rust::Yaml>{
    println!("{:?}", file);
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
            let socket_type = SupportedSockets::from_str(socket_definition["type"].as_str().unwrap()).expect("Unsupported socket type in schema definition, ignoring.");
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
