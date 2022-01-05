use crate::utils::bind_socket;

struct RouterClient {
    socket: zmq::Socket
}

impl RouterClient {
    fn new() -> RouterClient {
        RouterClient {
            socket: bind_socket(zmq::SocketType::DEALER, "tcp://*:5559").expect("failed to bind ROUTER client")
        }
    }

    fn mediate(&self) {
        loop {
            let mut message_buffer: zmq::Message = zmq::Message::new();
            match self.socket.poll(zmq::POLLIN, 2000).expect("Failed to poll socket") {
                0 => {break;} // nothing in the socket
                _ => {
                    self.socket.recv(&mut message_buffer, 0).expect("Failed to read message into buffer");
                    let message: String = "self.perform_task(message_buffer.as_str().unwrap());".to_string();
                    self.socket.send(&message, 0).expect("Sending message to req client failed");
                }
            }
        }
    }
}

pub fn run() {
    let dealer_client: RouterClient = RouterClient::new();
    dealer_client.mediate();
}