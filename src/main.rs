mod cliargs;
mod client;
mod communication;
mod server;

use std::{fs::File, io};

use clap::Parser;
use client::Oxyclient;

fn fetch_file(path: &std::path::PathBuf) -> File {
    todo!()
}

fn main() -> io::Result<()> {
    match cliargs::Cli::parse().command {
        cliargs::Command::Listen(listen_args) => {
            let address = listen_args.address;
            let _ = server::start_server(address); // TODO: error handling
        }

        cliargs::Command::Connect(connect_args) => {
            let mut client = Oxyclient::default();
            let _ = client.connect_to(connect_args.address); // TODO: error handling
                                                             // TODO: open interactive interface
        }

        cliargs::Command::Ping(ping_args) => {
            let client = Oxyclient::new(Some(ping_args.address));
            match client.ping_active_connection() {
                Ok(_) => println!("ping succeeded"),
                Err(e) => eprintln!("ping failed\n{}", e),
            }
        }

        cliargs::Command::SendFile(send_file_args) => {
            let client = Oxyclient::new(Some(send_file_args.address));
            let path = send_file_args.file_path;
            let file = fetch_file(&path);
            println!("sending over file {}", path.display());
            client.send_file(file);
            // TODO: some sort of generalized 'client::issue_command()' ?
        }
    }

    Ok(())
}
