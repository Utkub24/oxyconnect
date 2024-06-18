use std::fs::File;
use std::io::{self, Error, ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpStream};

use crate::communication;

#[derive(Default)]
pub struct Oxyclient {
    active_address: Option<SocketAddr>,
}

impl Oxyclient {
    pub fn new(address: Option<SocketAddr>) -> Self {
        Self {
            active_address: address,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.active_address.is_some()
    }

    pub fn connect_to(&mut self, address: SocketAddr) -> io::Result<()> {
        if self.is_connected() {
            // redundant check?
            let _ = self.disconnect();
        }
        self.active_address = Some(address);
        let stream = TcpStream::connect(address)?;
        Ok(())
    }

    pub fn disconnect(&mut self) -> io::Result<()> {
        match self.active_address {
            Some(_) => {
                // TODO: close active streams if any
                self.active_address = None;
                Ok(())
            }
            None => Err(Error::new(ErrorKind::NotConnected, "no active connection")),
        }
    }

    pub fn ping_active_connection(&self) -> io::Result<()> {
        match self.active_address {
            Some(address) => {
                let mut stream = TcpStream::connect(address)?;
                let message = "Ping!";
                println!("sending message: {}", message);
                stream.write(message.as_bytes())?;
                Ok(())
            }
            None => Err(Error::new(ErrorKind::NotConnected, "no active connection")),
        }
    }

    pub fn send_file(&self, mut file: File) -> io::Result<()> {
        match self.active_address {
            Some(address) => {
                let mut stream = TcpStream::connect(address)?;
                let mut buf = Vec::new();
                file.read_to_end(&mut buf)?; // TODO: this is inefficient
                stream.write_all(&buf)?;
                Ok(())
            }
            None => Err(Error::new(ErrorKind::NotConnected, "no active connection")),
        }
    }
}
