extern crate clap;
mod request;
mod reply;
mod utils;

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
    
    if matches.occurrences_of("server") > 0 {
        reply::run();
    } else {
        request::run();
    }
}
