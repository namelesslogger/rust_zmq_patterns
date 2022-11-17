use crate::utils::connect_socket;

struct DealerClient {
    socket: zmq::Socket,
}

impl DealerClient {
    fn new() -> DealerClient {
        DealerClient {
            socket: connect_socket(zmq::SocketType::DEALER, "tcp://localhost:5559")
                .expect("failed to connect DEALER client"),
        }
    }

    fn accept_connections(&self, message: &str) {
        let mut message_buffer: zmq::Message = zmq::Message::new();
        self.socket
            .send(message, 0)
            .expect("Sending message to rep client failed");

        match self
            .socket
            .poll(zmq::POLLIN, 2000)
            .expect("Failed to poll socket")
        {
            0 => {
                println!("Server is being awful quiet...");
            }
            _ => {
                self.socket
                    .recv(&mut message_buffer, 0)
                    .expect("Failed to read message into buffer");
                println!("Reply from server: {:?}", message_buffer.as_str().unwrap());
            }
        }
    }
}

pub fn run() {
    let dealer_client: DealerClient = DealerClient::new();
    dealer_client.accept_connections("hello there from a random dealer");
}
