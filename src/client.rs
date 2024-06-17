use std::io::Write;
use std::net::{SocketAddr, TcpStream};

use crate::communication;

pub fn ping_address(address: SocketAddr) {
    let mut stream = TcpStream::connect(address).expect("could not establish connection!");
    let message = "Ping!";
    println!("sending message: {}", message);
    stream.write(message.as_bytes()).expect("ping failed"); // TODO: don't panic
}

pub fn send_file(path: &std::path::PathBuf, stream: TcpStream) {
    communication::send_file(path, stream);
}
