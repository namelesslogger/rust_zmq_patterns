use crate::sockets::SocketModel;
use text_io::read;

struct ReqClient {
    socket: zmq::Socket,
    timeout: i64,
}

impl SocketModel for ReqClient {
    fn send(&self, message: &str) {
        self.socket
            .send(message, 0)
            .expect("Sending message to rep client failed");
    }

    fn recieve(&self) {
        let mut message_buffer: zmq::Message = zmq::Message::new();
        match self
            .socket
            .poll(zmq::POLLIN, self.timeout)
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

impl ReqClient {
    fn new() -> Self {
        ReqClient {
            socket: Self::connect_socket(zmq::SocketType::REQ, "tcp://localhost:5559")
                .expect("connection went off without a hitch"),
            timeout: 2000,
        }
    }
}

pub fn run() {
    let req_client: ReqClient = ReqClient::new();
    loop {
        let message: String = read!("{}\n");
        req_client.send(&message);
        req_client.recieve();
        if message == "END" {
            break;
        }
    }
    // why do I need this...
    std::process::exit(0);
}
