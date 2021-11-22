use crate::utils::bind_socket;

struct RepClient {
    socket: zmq::Socket,
    timeout: i64
}

impl RepClient {
    fn new() -> RepClient {
        RepClient {
            socket: bind_socket(zmq::SocketType::REP, "tcp://*:5559").expect("Failed to bind Reply socket type to"),
            timeout: 2000
        }
    }

    fn reply(&self, message: &str) {
        let mut message_buffer: zmq::Message = zmq::Message::new();
        
        loop {
            match self.socket.poll(zmq::POLLIN, self.timeout) {
                Ok(i) => {
                    match i {
                        0 => {continue;} // nothing in the socket
                        _ => {
                            self.socket.recv(&mut message_buffer, 0).expect("Failed to read message into buffer");
                            println!("Request from client: {:?}", message_buffer.as_str().unwrap());
                            self.socket.send(message, 0).expect("Sending message to req client failed");
                        }
                    }
                }
                Err(_) => {
                    println!("Nothin to do, passing on")
                } 
            }
        }
    }
}

pub fn run() {
    println!("Starting server process");
    let rep_client: RepClient = RepClient::new();
    rep_client.reply("Ohhhh a request!!!");

    // why do I need this...
    std::process::exit(0);
}