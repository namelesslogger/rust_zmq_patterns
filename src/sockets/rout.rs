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
        println!("hello");
    }
}

pub fn run() {
    let dealer_client: RouterClient = RouterClient::new();
    dealer_client.mediate();
}