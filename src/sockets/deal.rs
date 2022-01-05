use crate::utils::connect_socket;

struct DealerClient {
    socket: zmq::Socket
}

impl DealerClient {
    fn new() -> DealerClient {
        DealerClient {
            socket: connect_socket(zmq::SocketType::DEALER, "tcp://*:5559").expect("failed to connect DEALER client")
        }
    }

    fn accept_connections(&self) {
        println!("hello i am a dealer")
    }
}

pub fn run() {
    let dealer_client: DealerClient = DealerClient::new();
    dealer_client.accept_connections();
}