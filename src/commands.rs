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
        todo!()
    }
}
impl Decode {
    fn exec(self) {
        todo!()
    }
}

impl Remove {
    fn exec(self) {
        todo!()
    }
}

impl Print {
    fn exec(self) {
        todo!()
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

fn main() {
    let cli = Cli::parse();

    cli.command.delegate();
}

// fn
