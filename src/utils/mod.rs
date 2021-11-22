use zmq;

fn create_socket(s_type: zmq::SocketType) -> Result<zmq::Socket, zmq::Error> {
    let context = zmq::Context::new();
    context.socket(s_type)
}

pub fn bind_socket(s_type: zmq::SocketType, endpoint: &str) -> Result<zmq::Socket, zmq::Error> {
    match create_socket(s_type) {
        Ok(s) => match s.bind(endpoint) {
            Ok(_) => Ok(s),
            Err(err) => Err(err) 
        },
        Err(err) => Err(err)
    }
}

pub fn connect_socket(s_type: zmq::SocketType, endpoint: &str) -> Result<zmq::Socket, zmq::Error> {
    match create_socket(s_type) {
        Ok(s) => match s.connect(endpoint) {
            Ok(_) => Ok(s),
            Err(err) => Err(err)
        },
        Err(err) => Err(err)
    }
}