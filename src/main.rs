#![allow(dead_code, unused_variables)]

use cli::Cli;
mod chunk;
mod chunk_type;
mod cli;
mod commands;
mod png;

fn main() -> Result<(), anyhow::Error> {
    Cli::run()
}
