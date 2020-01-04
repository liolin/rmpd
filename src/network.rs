use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use log::info;

use super::mpd;

pub struct Server {
    local_peer: TcpListener,
}

impl Server {
    pub fn new(address: String) -> Server {
	Server {
	    local_peer: TcpListener::bind(address).unwrap()
	}
    }

    pub fn serve(&self) -> std::io::Result<()> {
	for stream in self.local_peer.incoming() {
	    self.handle_client(stream?);
	    break;
	}
	Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) {
	info!("New connection: {}", stream.peer_addr().unwrap());

	let welcome_message = format!("OK MPD {}", mpd::MPD_VERSION);
	stream.write(welcome_message.as_bytes()).unwrap();

	unimplemented!();
	loop {
	    let request = mpd::Parser::parse(&mut stream).unwrap();
	    info!("MPD Request: {}", request);
	}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_new_server() {
	let _server = Server::new("127.0.0.1:6666".to_string());
	
    }

    #[test]
    #[should_panic]
    fn test_create_new_server_with_invalid_address() {
	let _server = Server::new("1248.1.545.12.54:878787847".to_string());
    }

    #[test]
    #[ignore]
    fn test_right_welcome_message() {
	let server = Server::new("127.0.0.1:6667".to_string());

	use std::thread;
	let handler = thread::spawn(|| {
	    let mut client = TcpStream::connect("127.0.0.1:6667").unwrap();
	    let mut buffer = String::new();
	    client.read_to_string(&mut buffer).unwrap();
	    
	    assert_eq!("OK MPD 0.16.0", buffer.as_str());

	    client.write("status".as_bytes()).unwrap();
	});

	server.serve().unwrap();
	handler.join().unwrap();
    }
}
