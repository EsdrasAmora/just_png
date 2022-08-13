use std::fs::OpenOptions;

use anyhow::Context;
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

mod impls {
    use std::{
        borrow::Borrow,
        ffi::OsStr,
        fmt::format,
        fs,
        io::{ErrorKind, Write},
        path::{Path, PathBuf},
    };

    use anyhow::{bail, ensure};

    use super::*;
    use crate::{
        chunk::{self, Chunk},
        png::{self, Png},
    };

    impl Encode {
        pub(crate) fn exec(self) -> Result<(), anyhow::Error> {
            let a = fs::read("/Users/taqtile/Downloads/rustacean-flat-gesture.png")?;

            let mut png: Png = a.as_slice().try_into()?;
            let new_chunk: Chunk =
                Chunk::new("ruSt".try_into()?, "This is a secret message".into());
            png.append_chunk(new_chunk);

            let mut file = match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open("/Users/taqtile/Downloads/rustacean-flat-gesture(1).png")
            {
                Ok(file) => file,
                Err(ref err) if err.kind() == ErrorKind::AlreadyExists => {
                    bail!("file already exists")
                }
                Err(e) => panic!("Can't read from file: {}, err {}", "filenametodo", e),
            };

            let write = file.write_all(&png.as_bytes())?;

            Ok(())
        }

        fn handle_write_file<P: AsRef<Path>>(
            filename: P,
            content: &[u8],
        ) -> Result<(), anyhow::Error> {
            let mut max_retries = 10;
            let mut filename = filename.as_ref().to_owned();
            let stem: PathBuf = filename.file_stem().context("invalid file path")?.into();
            let extension: PathBuf = filename
                .extension()
                .context("invalid file format: file does not contain a extension")?
                .into();

            ensure!(
                extension.as_os_str() == "png",
                "invalid file format: file must be a png"
            );
            loop {
                // match OpenOptions::new()
                //     .write(true)
                //     .create_new(true)
                //     .open("/Users/taqtile/Downloads/rustacean-flat-gesture(1).png")
                // {
                //     Ok(mut file) => {
                //         file.write_all(content)?;
                //         break;
                //     }
                //     Err(ref err) if err.kind() == ErrorKind::AlreadyExists => {
                //         bail!("file {:?} already exists", filename.as_os_str())
                //     }
                //     Err(e) => {
                //         return Err(e).context(format!(
                //             "something wrent wrong wile trying to open the file {:?}",
                //             filename.as_os_str()
                //         ))
                //     }
                // };
                if let Ok(mut file) = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&filename)
                {
                    file.write_all(content)?;
                    break;
                }
                max_retries -= 1;
                if max_retries == 0 {
                    break;
                }

                filename = stem
                    .join(format!("({}) ", 10 - max_retries))
                    .join(".")
                    .join(&extension);
            }
            println!("written result to file {:?}", filename);
            Ok(())
        }
    }
    impl Decode {
        pub(crate) fn exec(self) -> Result<(), anyhow::Error> {
            println!("you runned the command Decode with args {:?}", self);
            Ok(())
        }
    }

    impl Remove {
        pub(crate) fn exec(self) -> Result<(), anyhow::Error> {
            println!("you runned the command Remove with args {:?}", self);
            Ok(())
        }
    }

    impl Print {
        pub(crate) fn exec(self) -> Result<(), anyhow::Error> {
            println!("you runned the command Print with args {:?}", self);
            Ok(())
        }
    }
}
