extern crate clap;
mod request;
mod reply;
mod subscribe;
mod publish;
mod utils;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Zmq Test bed")
        .version("0.1")
        .author("Shane M <johnshanie@protonmail.com>")
        .about("I do things")
        .arg(Arg::with_name("arch")
            .short("a")
            .long("arch")
            .help("determines if the architure is pubsub or request reply"))
        .arg(Arg::with_name("server")
            .short("s")
            .long("server")
            .help("Sets process to act as server or client"))
        .get_matches();
    
    if matches.occurrences_of("server") > 0 {
        if matches.occurrences_of("arch") > 0 {
            publish::run();
        } else {
            reply::run();
        }
    } else {
        if matches.occurrences_of("arch") > 0 {
            subscribe::run();
        } else {
            request::run();
        }
    }
}
