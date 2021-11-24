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
}

pub fn run() {
    let dealer_client: RouterClient = RouterClient::new();
}