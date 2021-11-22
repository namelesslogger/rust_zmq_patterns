use crate::utils::bind_socket;
use std::{thread, time};

struct PubClient {
    socket: zmq::Socket
}

impl PubClient {
    fn new() -> PubClient {
        PubClient {
            socket: bind_socket(zmq::SocketType::PUB, "tcp://*:5559").expect("failed to bind Publisher client")
        }
    }

    fn publish_data(&self, message: &str) {
        for _ in 0..30 {
            self.socket.send(message, 0).expect("Failed to send any data");
            println!("Sent some messafes out to anyone listening");
            thread::sleep(time::Duration::from_secs(1));
        }

        self.terminate_subscribers();
    }

    fn terminate_subscribers(&self) {
        self.socket.send("END", 0).expect("failed to send termination string, process wont stop!")
    }
}

pub fn run() {
    println!("Starting publiisher process process");
    let pub_client: PubClient = PubClient::new();
    pub_client.publish_data("this is a random message being sent 100 times");
    
    // why do I need this...
    std::process::exit(0);
}