use crate::utils::{bind_socket, connect_socket};
use zmq;

struct ReqClient {
    socket: zmq::Socket
}

struct RepClient {
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

pub fn run(server: bool) {
    if server {
        println!("Starting server process");
        let rep_client: RepClient = RepClient::new();
        rep_client.reply("I got a Messsage!!! Heres the response");
    } else {
        println!("Starting client process");
        let req_client: ReqClient = ReqClient::new();
        req_client.request("Ohhhh a request!!!");
    }
}