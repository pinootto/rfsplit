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
    /// input file (without the -sequence number)
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
    let mut output_file = File::create(args.output_file)?;

    for i in 0..args.num_of_files {
        let mut input_file = File::open(format!("{}-{}", args.input_file.as_str(), i))?;
        let mut buf = Vec::new();
        let file_size = input_file.read_to_end(&mut buf)?;
        println!("file_size {} = {}", i, file_size);
        output_file.write_all(buf.as_slice())?;
    }
    Ok(())
}
