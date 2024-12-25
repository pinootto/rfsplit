use clap::Parser;
use std::io::prelude::*;
use std::{fs::File, io::Read};

/// A simple program to join multiple smaller files into a large file
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "Join a series of small files into a large file"
)]
struct Args {
    /// input file
    #[arg(short, long)]
    input_file: String,

    /// output file
    #[arg(short, long, default_value = "/tmp/rfjoin.out")]
    output_file: String,

    /// number of small files
    #[arg(short, long)]
    num_of_files: usize,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    Ok(())
}
