use crate::utils::connect_socket;

struct ReqClient {
    socket: zmq::Socket,
    timeout: i64
}

impl ReqClient {
    fn new() -> ReqClient {
        ReqClient {
            socket: connect_socket(zmq::SocketType::REQ, "tcp://localhost:5559").unwrap(),
            timeout: 2000
        }
    }

    fn request(&self, message: &str) {
        let mut message_buffer: zmq::Message = zmq::Message::new();
        self.socket.send(message, 0).unwrap();

        match self.socket.poll(zmq::POLLIN, self.timeout) {
            Ok(i) => {
                //  can only be 0 (nothing) or 1 at the point
                match i {
                    0 => {
                        println!("Server is being awful quiet...");
                    },
                    _ => {
                        self.socket.recv(&mut message_buffer, 0).unwrap();
                        println!("Reply from server: {:?}", message_buffer.as_str().unwrap());
                    }
                }
            },
            Err(_) => {
                println!("What in the hell happened during polling!")
            }
        }
    }
}

pub fn run() {
    println!("Starting client process");
    let req_client: ReqClient = ReqClient::new();
    req_client.request("Ohhhh a request!!!");
    
    // why do I need this...
    std::process::exit(0);
}