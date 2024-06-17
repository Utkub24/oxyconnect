use std::{net::SocketAddr, path::PathBuf};

use clap::{command, Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about = "Client for interacting with Oxyserver")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Start listening on the specified address")]
    Listen(ListenArgs),

    #[command(about = "Ping the specified address")]
    Ping(PingArgs),

    #[command(about = "Send a file to the server")]
    SendFile(SendFileArgs),
}

#[derive(Args, Debug)]
pub struct SendFileArgs {
    #[arg(short = 'f', long = "file", help = "Path to the file to send")]
    pub file_path: PathBuf
}

#[derive(Args, Debug)]
pub struct ListenArgs {
    #[arg(short = 'a', long = "address", help = "Address of the server")]
    pub address: SocketAddr
}

#[derive(Args, Debug)]
pub struct PingArgs {
    #[arg(short = 'a', long = "address", help = "Address of the server")]
    pub address: SocketAddr
}