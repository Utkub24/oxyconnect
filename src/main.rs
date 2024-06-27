mod cliargs;
mod client;
mod communication;
mod server;
mod repl;

use std::{fs::{File, OpenOptions}, io};

use clap::Parser;
use client::Oxyclient;
use repl::Repl;
use server::Oxyserver;

fn fetch_file(path: &std::path::PathBuf) -> io::Result<File> {
    OpenOptions::new().read(true).open(path)
}

fn main() -> io::Result<()> {
    match cliargs::Cli::parse().command {
        cliargs::Command::Listen(listen_args) => match Oxyserver::create_at(listen_args.address) {
            Ok(server) => server.start().expect(""),
            Err(e) => eprintln!("failed to create server: {}", e),
        },

        cliargs::Command::Connect(connect_args) => {
            let mut client = Oxyclient::default();
            let _ = client.bind(connect_args.address); // TODO: error handling

            let mut repl = Repl::new(&mut client);
            repl.start();
        }

        cliargs::Command::Ping(ping_args) => {
            let client = Oxyclient::new(ping_args.address);
            match client.ping() {
                Ok(_) => println!("ping succeeded"),
                Err(e) => eprintln!("ping failed\n{}", e),
            }
        }

        cliargs::Command::SendFile(send_file_args) => {
            let client = Oxyclient::new(send_file_args.address);
            let path = &send_file_args.file_path;
            let file = fetch_file(path)?;
            println!("sending over file {}", path.display());
            match client.send_file(file) {
                Ok(_) => println!("sent!"),
                Err(e) => eprintln!("failed to send file:\n{}", e),
            }
            // TODO: some sort of generalized 'client::issue_command()' ?
        }
    }

    Ok(())
}
