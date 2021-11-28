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

    fn reply(&self) {
        let mut message_buffer: zmq::Message = zmq::Message::new();
        
        loop {
            match self.socket.poll(zmq::POLLIN, self.timeout) {
                Ok(i) => {
                    match i {
                        0 => {continue;} // nothing in the socket
                        _ => {
                            self.socket.recv(&mut message_buffer, 0).expect("Failed to read message into buffer");
                            println!("Performing some task for workers:");
                            let message: String = self.perform_task();
                            self.socket.send(&message, 0).expect("Sending message to req client failed");
                        }
                    }
                }
                Err(_) => {
                    println!("Nothin to do, passing on")
                } 
            }
        }
    }

    fn perform_task(&self) -> String {
        "task_result".to_string()
    }
}

pub fn run() {
    println!("Starting server process");
    let rep_client: RepClient = RepClient::new();
    rep_client.reply();

    // why do I need this...
    std::process::exit(0);
}