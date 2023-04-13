mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::{Cli, Command};
use clap::Parser;

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
        }
        Command::Decode { file, chunk_type } => {
            println!("decode {:?} using type {}", file.as_path(), chunk_type);
        }
        Command::Remove { file, chunk_type } => {
            println!("remove {} from {:?}", chunk_type, file.as_path());
        }
        Command::Print { file } => {
            println!("print {:?}", file.as_path());
        }
    }

    Ok(())
}
