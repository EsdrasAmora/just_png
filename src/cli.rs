use crate::commands::{Decode, Encode, Print, Remove};
use clap::{Parser, Subcommand};

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
    pub fn run() -> Result<(), anyhow::Error> {
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

impl Commands {
    fn delegate(self) -> Result<(), anyhow::Error> {
        match self {
            Commands::Encode(args) => args.exec(),
            Commands::Decode(args) => args.exec(),
            Commands::Remove(args) => args.exec(),
            Commands::Print(args) => args.exec(),
        }
    }
}
