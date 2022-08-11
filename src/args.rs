use clap::{Args, Parser, Subcommand};

#[derive(Args)]
pub(crate) struct Encode {
    #[clap(value_parser)]
    path: String,

    #[clap(value_parser)]
    chunk_type: String,

    #[clap(value_parser)]
    message: String,

    #[clap(value_parser)]
    output: Option<String>,
}

#[derive(Args)]
pub(crate) struct Decode {
    #[clap(value_parser)]
    path: String,

    #[clap(value_parser)]
    chunk_type: String,
}

#[derive(Args)]
pub(crate) struct Remove {
    #[clap(value_parser)]
    path: String,

    #[clap(value_parser)]
    chunk_type: String,
}

#[derive(Args)]
pub(crate) struct Print {
    #[clap(value_parser)]
    path: String,
}
