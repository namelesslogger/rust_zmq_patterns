use crate::utils::bind_socket;

struct RepClient {
    socket: zmq::Socket
}

impl RepClient {
    fn new() -> RepClient {
        RepClient {
            socket: bind_socket(zmq::SocketType::REP, "tcp://*:5559").unwrap()
        }
    }

    fn reply(&self, message: &str) {
        let mut message_buffer: zmq::Message = zmq::Message::new();
        self.socket.recv(&mut message_buffer, 0).unwrap();
        println!("Request from client: {:?}", message_buffer.as_str().unwrap());
        self.socket.send(message, 0).unwrap();
    }
}

pub fn run() {
    println!("Starting server process");
    let rep_client: RepClient = RepClient::new();
    rep_client.reply("Ohhhh a request!!!");
}