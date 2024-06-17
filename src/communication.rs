use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    net::{Shutdown, TcpStream},
    path::Path,
};

pub fn send_file(path: &Path, mut stream: TcpStream) {
    match OpenOptions::new().read(true).open(path) {
        Ok(mut file) => {
            let mut buf = Vec::new();
            let read_size = read_file_to_buffer(&mut file, &mut buf).unwrap_or(0);
            println!("read {} bytes into buffer", read_size);

            match write_all_to_stream(&mut stream, &buf) {
                Ok(_) => println!("successfully sent file!"),
                Err(_) => {
                    let _ = stream.shutdown(Shutdown::Both);
                }
            }
        }
        Err(e) => {
            eprintln!("error opening file: {}", path.display());
            eprintln!("{}", e);
        }
    }
}

pub fn read_file_to_buffer(file: &mut File, buf: &mut Vec<u8>) -> io::Result<usize> {
    file.read_to_end(buf).inspect_err(|e| {
        eprintln!("error while reading from file:");
        eprintln!("file descriptor: {:?}", file);
        eprintln!("{}", e);
    })
}

pub fn write_all_to_stream(stream: &mut TcpStream, buf: &[u8]) -> io::Result<()> {
    stream.write_all(buf).inspect_err(|e| {
        eprintln!("error while writing to socket");
        eprintln!("socket: {:?}", stream);
        eprintln!("{}", e);
    })
}
