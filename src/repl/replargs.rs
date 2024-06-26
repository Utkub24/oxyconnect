use std::{net::SocketAddr, path::PathBuf};

use clap::{command, Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about = "REPL client for interacting with Oxyserver", name = "")]
pub struct ReplParser {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Print status")]
    Status,

    #[command(about = "Connect to the specified address")]
    Connect(ConnectArgs),

    #[command(about = "Disconnect from the server")]
    Disconnect,

    #[command(about = "Ping the active connection")]
    Ping,

    #[command(about = "Send a file to the server")]
    SendFile(SendFileArgs),
}

#[derive(Args, Debug)]
pub struct SendFileArgs {
    #[arg(short = 'f', long = "file", help = "Path to the file to send")]
    pub file_path: PathBuf
}

#[derive(Args, Debug)]
pub struct ConnectArgs {
    #[arg(short = 'a', long = "address", help = "Address of the server")]
    pub address: SocketAddr
}
