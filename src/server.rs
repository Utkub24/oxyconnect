use std::{
    fs::File,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::{Path, PathBuf}, str::FromStr,
};

pub struct Oxyserver {
    active_address: SocketAddr,
    listener: TcpListener,
}

impl Oxyserver {
    pub fn create_at(address: SocketAddr) -> std::io::Result<Self> {
        Ok(Self {
            active_address: address,
            listener: TcpListener::bind(address)?,
        })
    }

    pub fn stop(&mut self) {
        todo!("stop server and clean up")
    }

    pub fn start(&self) -> std::io::Result<()> {
        let address = &self.active_address;
        let listener = &self.listener;
        println!("started listening on {}", address);
        for connection in listener.incoming() {
            println!("establishing new connection: {:?}", connection); // TODO: remove debug printing
                                                                       // NOTE: TcpListener::incoming() never returns none
            match self.handle_connection(connection?) {
                Ok(_) => println!("connection handled\n"),
                Err(e) => eprintln!("error when handling connection:\n{}", e),
            } // TODO: spawn thread for each connection
        }

        Ok(())
    }

    fn handle_connection(&self, mut stream: TcpStream) -> std::io::Result<()> {
        let mut buf = String::new();
        stream.read_to_string(&mut buf)?;
        println!("received message: {}", buf);
        let path = PathBuf::from_str("recieved_file").unwrap();
        println!("writing to file: {}", path.display());
        match File::create(&path) {
            Ok(mut file) => {
                match file.write_all(buf.as_bytes()) {
                    Ok(_) => println!("wrote to file: {}", path.canonicalize().unwrap().display()),
                    Err(e) => eprintln!("error writing to file: {}", e),
                }
            }
            Err(e) => eprintln!("error creating/opening file: {}", e),
        }

        // TODO: async pong client

        Ok(())
    }
}
