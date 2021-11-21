use crate::utils::connect_socket;

struct ReqClient {
    socket: zmq::Socket
}

impl ReqClient {
    fn new() -> ReqClient {
        ReqClient {
            socket: connect_socket(zmq::SocketType::REQ, "tcp://localhost:5559").unwrap(),
        }
    }

    fn request(&self, message: &str) {
        let mut message_buffer: zmq::Message = zmq::Message::new();
        self.socket.send(message, 0).unwrap();
        self.socket.recv(&mut message_buffer, 0).unwrap();
        println!("Reply from server: {:?}", message_buffer.as_str().unwrap());
    }
}

pub fn run() {
    println!("Starting client process");
    let req_client: ReqClient = ReqClient::new();
    req_client.request("Ohhhh a request!!!");
}