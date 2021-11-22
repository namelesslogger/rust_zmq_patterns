extern crate clap;
mod request;
mod reply;
mod subscribe;
mod publish;
mod utils;

use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("architecture.yaml");
    let matches = App::from(yaml).get_matches();
    
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
