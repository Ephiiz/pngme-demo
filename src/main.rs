mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use std::str::FromStr;

use args::{Cli, Command};
use clap::Parser;
use png::Png;

use crate::{chunk::Chunk, chunk_type::ChunkType};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Encode {
            file,
            chunk_type,
            message,
            output_file,
        } => {
            println!(
                "encode {:?} using type {} for message {} to file {:?}",
                file.as_path(),
                chunk_type,
                message,
                output_file
            );
            let pic = std::fs::read(&file)?;
            let mut png = Png::try_from(pic.as_slice())?;
            png.append_chunk(Chunk::new(
                ChunkType::from_str(chunk_type.as_str())?,
                message.into_bytes(),
            ));
            if output_file.is_some() {
                std::fs::write(output_file.unwrap(), png.as_bytes())?;
            } else {
                std::fs::write(&file, png.as_bytes())?;
            }
        }
        Command::Decode { file, chunk_type } => {
            println!("decode {:?} using type {}", file.as_path(), chunk_type);
            let pic = std::fs::read(file)?;
            let png = Png::try_from(pic.as_slice())?;
            let out = png.chunk_by_type(chunk_type.as_str());
            if out.is_some() {
                println!("{}", out.unwrap());
            } else {
                println!("No chunk of type {} found", chunk_type);
            }
        }
        Command::Remove { file, chunk_type } => {
            println!("remove {} from {:?}", chunk_type, file.as_path());
            let pic = std::fs::read(&file)?;
            let mut png = Png::try_from(pic.as_slice())?;
            let out = png.remove_chunk(chunk_type.as_str());
            if out.is_ok() {
                std::fs::write(&file, png.as_bytes())?;
                println!("{} removed from file {:?}", chunk_type, &file.as_path());
            } else {
                println!(
                    "failed to remove chunk of type {} from file, likely doesnt exist",
                    chunk_type
                );
            }
        }
        Command::Print { file } => {
            println!("print {:?}", file.as_path());
            let pic = std::fs::read(&file)?;
            let png = Png::try_from(pic.as_slice())?;
            println!("{}", png);
        }
    }

    Ok(())
}
