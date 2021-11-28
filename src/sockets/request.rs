use crate::utils::connect_socket;
use text_io::read;

struct ReqClient {
    socket: zmq::Socket,
    timeout: i64
}

impl ReqClient {
    fn new() -> ReqClient {
        ReqClient {
            socket: connect_socket(zmq::SocketType::REQ, "tcp://localhost:5559").expect("Failed to connect request client"),
            timeout: 2000
        }
    }

    fn request(&self, message: &str) {
        let mut message_buffer: zmq::Message = zmq::Message::new();
        self.socket.send(message, 0).expect("Sending message to rep client failed");

        match self.socket.poll(zmq::POLLIN, self.timeout).expect("Failed to poll socket") {
            0 => {
                println!("Server is being awful quiet...");
            },
            _ => {
                self.socket.recv(&mut message_buffer, 0).expect("Failed to read message into buffer");
                println!("Reply from server: {:?}", message_buffer.as_str().unwrap());
            }
        }
    }
}

pub fn run() {
    let req_client: ReqClient = ReqClient::new();
    loop {
        let message: String = read!("{}\n");
        req_client.request(&message);
        if message == "END" {
            break;
        }
    }
    // why do I need this...
    std::process::exit(0);
}