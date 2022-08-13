use crate::{chunk::Chunk, png::Png};
use anyhow::{bail, ensure, Context};
use clap::Args;
use std::{
    ffi::OsString,
    fs::{self, OpenOptions},
    io::{ErrorKind, Write},
    path::PathBuf,
};

#[derive(Args, Debug)]
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

#[derive(Args, Debug)]
pub(crate) struct Decode {
    #[clap(value_parser)]
    path: String,

    #[clap(value_parser)]
    chunk_type: String,
}

#[derive(Args, Debug)]
pub(crate) struct Remove {
    #[clap(value_parser)]
    path: String,

    #[clap(value_parser)]
    chunk_type: String,
}

#[derive(Args, Debug)]
pub(crate) struct Print {
    #[clap(value_parser)]
    path: String,
}

impl Encode {
    pub(crate) fn exec(self) -> Result<(), anyhow::Error> {
        let file = fs::read(&self.path)?;

        let mut png: Png = file.as_slice().try_into()?;
        let new_chunk: Chunk = Chunk::new(
            self.chunk_type.as_bytes().try_into()?,
            self.message.as_str().into(),
        );
        png.append_chunk(new_chunk);

        self.handle_write_file(&png.as_bytes())?;

        Ok(())
    }

    fn handle_write_file(&self, content: &[u8]) -> Result<(), anyhow::Error> {
        let mut max_retries = 10;
        let mut filename = PathBuf::from(self.output.as_ref().unwrap_or(&self.path));
        let stem: OsString = filename.file_stem().context("empty file name")?.into();
        let extension: OsString = filename
            .extension()
            .context("invalid file format: file does not contain a extension")?
            .into();

        ensure!(
            extension == "png",
            "invalid file format: file must be a png"
        );

        let max_size = filename.capacity() + "(10)".len();
        loop {
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&filename)//"/Users/taqtile/Downloads/rustacean-flat-gesture(1).png"
            {
                Ok(mut file) => {
                    file.write_all(content)?;
                    break;
                }
                Err(ref err) if err.kind() == ErrorKind::AlreadyExists => {
                    max_retries -= 1;

                    if max_retries == 0 {
                        bail!("could not write result to file")
                    }

                    let mut temp = OsString::with_capacity(max_size);
                    temp.push(&stem);
                    temp.push(format!("({})", 10 - max_retries));
                    temp.push(".");
                    temp.push(&extension);
                    filename = temp.into();
                }
                Err(e) => {
                    return Err(e).context(format!(
                        "something wrent wrong wile trying to create the file {:?}",
                        filename.as_os_str()
                    ))
                }
            };
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
