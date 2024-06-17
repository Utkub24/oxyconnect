use std::{
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
};

pub fn start_server(address: SocketAddr) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?; // TODO: error handling
    println!("started listening on {}", address);
    for connection in listener.incoming() {
        println!("establishing new connection:\n{:?}", connection); // TODO: remove debug printing
        match connection {
            Ok(c) => {
                println!("successfully connected!");
                let _ = handle_connection(c); // TODO: error handing, spawn new thread for each connection
            }
            Err(e) => {
                eprintln!("failed to establish connection");
                eprintln!("{}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = String::new();
    stream
        .read_to_string(&mut buf)
        .expect("reading from stream failed"); // TODO: don't panic on read fail
    println!("received message: {}", buf);

    // TODO: async pong client

    println!("connection handled");
    Ok(())
}
