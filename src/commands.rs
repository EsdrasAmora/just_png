use clap::{Args, Parser, Subcommand};

use crate::args::{Decode, Encode, Print, Remove};

#[derive(Parser)]
#[clap(
    author = "Esdras Amora",
    version = "0.0.1",
    about,
    propagate_version = true
)]
/// hide secret secret messages inside a png
pub(crate) struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn run() {
        let cli = Cli::parse();
        cli.command.delegate()
    }
}

#[derive(Subcommand)]
enum Commands {
    Encode(Encode),
    Decode(Decode),
    Remove(Remove),
    Print(Print),
}

impl Encode {
    fn exec(self) {
        println!("you runned the command Encode with args {:?}", self)
    }
}
impl Decode {
    fn exec(self) {
        println!("you runned the command Decode with args {:?}", self)
    }
}

impl Remove {
    fn exec(self) {
        println!("you runned the command Remove with args {:?}", self)
    }
}

impl Print {
    fn exec(self) {
        println!("you runned the command Print with args {:?}", self)
    }
}

impl Commands {
    fn delegate(self) {
        match self {
            Commands::Encode(args) => args.exec(),
            Commands::Decode(args) => args.exec(),
            Commands::Remove(args) => args.exec(),
            Commands::Print(args) => args.exec(),
        }
    }
}

fn handle_path() {
    todo!()
}
