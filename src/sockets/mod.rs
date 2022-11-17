pub mod deal;
pub mod publish;
pub mod reply;
pub mod request;
pub mod rout;
pub mod subscribe;

pub trait SocketModel {
    fn send(&self, message: &str) {}

    fn recieve(&self) {}

    fn create_socket(s_type: zmq::SocketType) -> Result<zmq::Socket, zmq::Error> {
        let context = zmq::Context::new();
        context.socket(s_type)
    }

    fn bind_socket(s_type: zmq::SocketType, endpoint: &str) -> Result<zmq::Socket, zmq::Error> {
        match Self::create_socket(s_type) {
            Ok(s) => match s.bind(endpoint) {
                Ok(_) => Ok(s),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }

    fn connect_socket(s_type: zmq::SocketType, endpoint: &str) -> Result<zmq::Socket, zmq::Error> {
        match Self::create_socket(s_type) {
            Ok(s) => match s.connect(endpoint) {
                Ok(_) => Ok(s),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }
}
