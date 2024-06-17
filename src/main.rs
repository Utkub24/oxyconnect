mod cliargs;
mod communication;
mod client;
mod server;

use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
};

use clap::Parser;

fn main() -> io::Result<()> {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    match cliargs::Cli::parse().command {
        cliargs::Command::Listen(listen_args) => {
            let address = listen_args.address;
            let _ = server::start_server(address); // TODO: error handling
        }

        cliargs::Command::Connect(connect_args) => {
            let address = connect_args.address;
            client::connect_to(address); // TODO: return connection handle
        }

        cliargs::Command::Ping(ping_args) => {
            let address = ping_args.address;
            client::ping_address(address);
        }

        cliargs::Command::SendFile(send_file_args) => {
            let stream = TcpStream::connect(socket)?;
            let path = send_file_args.file_path;
            println!("sending over file {}", path.display());
            client::send_file(&path, stream);
            // TODO: some sort of generalized 'client::issue_command()' ?
        }
    }

    Ok(())
}
