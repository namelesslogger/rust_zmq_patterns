use crate::utils::connect_socket;

struct SubClient {
    socket: zmq::Socket,
    topic: Vec<u8>,
    timeout: i64,
}

impl SubClient {
    fn new() -> SubClient {
        SubClient {
            socket: connect_socket(zmq::SocketType::SUB, "tcp://localhost:5559")
                .expect("Couldnt connect Subscriber socket"),
            topic: b"".to_vec(),
            timeout: 2000,
        }
    }

    // TODO add ability to add more topics and subscriptions
    fn subscribe(&self) {
        self.socket
            .set_subscribe(&self.topic)
            .expect("Couldn't subscribe!");
    }

    fn consume_subscription(&self) {
        let mut message_buffer: zmq::Message = zmq::Message::new();

        loop {
            match self
                .socket
                .poll(zmq::POLLIN, self.timeout)
                .expect("Failed to poll socket")
            {
                0 => {
                    //TODO add some concept of liveness here
                }
                _ => {
                    self.socket
                        .recv(&mut message_buffer, 0)
                        .expect("Failed to read message into buffer");
                    let published_message = message_buffer.as_str().unwrap();
                    if published_message == "END" {
                        break;
                    }
                    println!("Request recieved! {:?}", published_message);
                }
            }
        }
    }
}

pub fn run() {
    println!("Starting subscripber process");
    let sub_client: SubClient = SubClient::new();
    sub_client.subscribe();
    sub_client.consume_subscription();
}
