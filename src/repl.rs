mod replargs;

use crate::client::Oxyclient;
use clap::Parser;
use reedline::{DefaultPrompt, DefaultPromptSegment, Prompt, Reedline};
use replargs::ReplParser;
use std::{fs::OpenOptions, io, iter};

pub struct Repl<'a> {
    client: &'a mut Oxyclient,
    rl: Reedline,
    prompt: Box<dyn Prompt>,
}

enum ParsedLine {
    Command(replargs::Command),
    ClapError(clap::error::Error),
    ReadLineError(io::Error),
    Abort,
}

impl<'a> Repl<'a> {
    pub fn new(client: &'a mut Oxyclient) -> Self {
        let prompt = DefaultPrompt::new(
            DefaultPromptSegment::Basic("Oxyconnect >> ".to_owned()),
            DefaultPromptSegment::Empty,
        );
        Repl {
            client,
            rl: Reedline::create(),
            prompt: Box::new(prompt),
        }
    }

    pub fn start(&mut self) {
        loop {
            match self.read() {
                ParsedLine::Command(c) => self.eval(c),
                ParsedLine::ClapError(e) => e.print().unwrap(),
                ParsedLine::ReadLineError(e) => eprintln!("{}", e),
                ParsedLine::Abort => break,
            }
        }
    }

    fn read(&mut self) -> ParsedLine {
        match self.rl.read_line(&*self.prompt) {
            Ok(s) => match s {
                reedline::Signal::Success(buf) => Repl::parse_line(&buf),
                reedline::Signal::CtrlC | reedline::Signal::CtrlD => ParsedLine::Abort,
            },
            Err(e) => ParsedLine::ReadLineError(e),
        }
    }

    fn parse_line(line: &str) -> ParsedLine {
        match ReplParser::try_parse_from(iter::once("").chain(line.split_whitespace())) {
            Ok(c) => ParsedLine::Command(c.command),
            Err(e) => ParsedLine::ClapError(e),
        }
    }

    fn eval(&mut self, command: replargs::Command) {
        // happy !
        // issue command to server - match arg
        // meaning: wait for response from server
        match command {
            replargs::Command::Status => {
                if self.client.is_connected() {
                    println!("Connected to: {}", self.client.active_address().unwrap())
                } else {
                    println!("No active connection");
                }
            }
            replargs::Command::Connect(args) => self.client.connect_to(args.address).unwrap(),
            replargs::Command::Disconnect => self.client.disconnect().unwrap(),
            replargs::Command::Ping => self.client.ping_active_connection().unwrap(),
            replargs::Command::SendFile(args) => {
                match OpenOptions::new().read(true).open(args.file_path) {
                    Ok(file) => {
                        match self.client.send_file(file) {
                            Ok(_) => println!("sent!"),
                            Err(e) => eprintln!("failed to send file:\n{}", e),
                        }
                    }
                    Err(e) => eprintln!("error opening file:\n{}", e),
                }
            }
        }
    }
}
