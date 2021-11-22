use crate::utils::connect_socket;

struct SubClient {
    socket: zmq::Socket,
    timeout: i64
}

impl SubClient {
    fn new() -> SubClient {
        SubClient {
            socket: connect_socket(zmq::SocketType::SUB, "tcp://localhost:5559").unwrap(),
            timeout: 2000
        }
    }

    fn consume_subscription(&self) {
        let mut message_buffer: zmq::Message = zmq::Message::new();

        match self.socket.poll(zmq::POLLIN, self.timeout) {
            Ok(i) => {
                match i {
                    0 => {println!("Got nuthin")} // nothing in the socket
                    _ => {
                        self.socket.recv(&mut message_buffer, 0).unwrap();
                        println!("Request recieved! {:?}", message_buffer.as_str().unwrap());
                    }
                }
            }
            Err(_) => {
                println!("Nothin to do, passing on")
            } 
        }
    }
}

pub fn run() {
    println!("Starting subscripber process");
    let sub_client: SubClient = SubClient::new();
    sub_client.consume_subscription();
    
    // why do I need this...
    std::process::exit(0);
}