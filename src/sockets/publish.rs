use crate::utils::bind_socket;
use text_io::read;

struct PubClient {
    socket: zmq::Socket
}

impl PubClient {
    fn new() -> PubClient {
        PubClient {
            socket: bind_socket(zmq::SocketType::PUB, "tcp://*:5559").expect("failed to bind Publisher client")
        }
    }

    fn publish_data(&self, message: &str) {
        self.socket.send(message, 0).expect("Failed to send any data");
    }

    fn terminate_subscribers(&self) {
        self.socket.send("END", 0).expect("failed to send termination string, process wont stop!")
    }
}

pub fn run() {
    let pub_client: PubClient = PubClient::new();
    loop {
        let message: String = read!("{}\n");
        if message == "END" {
            pub_client.terminate_subscribers();
            break;
        }
        pub_client.publish_data(&message);
    }
}