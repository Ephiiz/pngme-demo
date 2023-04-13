use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Subcommand)]
pub enum Command {
    Encode {
        file: PathBuf,
        chunk_type: String,
        message: String,
        output_file: Option<PathBuf>,
    },

    Decode {
        file: PathBuf,
        chunk_type: String,
    },

    Remove {
        file: PathBuf,
        chunk_type: String,
    },

    Print {
        file: PathBuf,
    },
}
