#![allow(dead_code, unused_imports, unused_variables)]

use commands::Cli;
mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

fn main() -> Result<(), anyhow::Error> {
    Cli::run()
}
