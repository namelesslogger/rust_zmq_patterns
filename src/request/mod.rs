use crate::utils::{bind_socket, connect_socket};
use zmq;

pub struct ReqClient {
    socket: zmq::Socket
}

pub struct RepClient {
    socket: zmq::Socket
}

impl ReqClient {
    pub fn new() -> ReqClient {
        ReqClient {
            socket: connect_socket(zmq::SocketType::REQ, "tcp://localhost:5559").unwrap(),
        }
    }

    pub fn request(&self, message: &str) {
        let mut message_buffer: zmq::Message = zmq::Message::new();
        self.socket.send(message, 0).unwrap();
        self.socket.recv(&mut message_buffer, 0).unwrap();
        println!("Reply from server: {:?}", message_buffer.as_str().unwrap());
    }
}

impl RepClient {
    pub fn new() -> RepClient {
        RepClient {
            socket: bind_socket(zmq::SocketType::REP, "tcp://*:5559").unwrap()
        }
    }

    pub fn reply(&self, message: &str) {
        let mut message_buffer: zmq::Message = zmq::Message::new();
        self.socket.recv(&mut message_buffer, 0).unwrap();
        println!("Request from client: {:?}", message_buffer.as_str().unwrap());
        self.socket.send(message, 0).unwrap();
    }
}
