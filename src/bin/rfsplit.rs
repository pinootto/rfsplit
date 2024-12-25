use clap::Parser;
use std::io::prelude::*;
use std::{fs::File, io::Read};

/// A simple program to split a large file into multiple smaller files
#[derive(Parser, Debug)]
#[command(version, about, long_about = "Split large file into small files")]
struct Args {
    /// input file
    #[arg(short, long)]
    input_file: String,

    /// output file
    #[arg(short, long, default_value = "/tmp/rfsplit.out")]
    output_file: String,

    /// size of each small file
    #[arg(short, long, default_value_t = 10000000)]
    chunk_size: usize,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let mut input_file = File::open(args.input_file)?;
    let file_size = input_file.metadata().unwrap().len();
    // println!("file size = {}", file_size);

    let chunk_size: usize = args.chunk_size;
    // println!("chunk_size = {}", chunk_size);

    let chunk_num = (file_size / chunk_size as u64).try_into().unwrap();
    // println!("chunks = {}", chunk_num);

    let remain: usize = (file_size % chunk_size as u64).try_into().unwrap();
    // println!("remaning bytes = {}", remain);

    let mut buf = vec![0; chunk_size];

    for i in 0..chunk_num {
        let _ = input_file.read_exact(&mut buf)?;

        // let output_filename = String::from(args.output_file.as_str()) + &i.to_string();
        let output_filename = format!("{}-{}", args.output_file.as_str(), i);
        let mut output_file = File::create(output_filename)?;
        output_file.write_all(buf.as_slice())?;
    }

    if remain > 0 {
        buf.truncate(remain);
        let _ = input_file.read_exact(&mut buf)?;

        // let output_filename = String::from(args.output_file.as_str()) + &chunk_num.to_string();
        let output_filename = format!("{}-{}", args.output_file.as_str(), chunk_num);
        let mut output_file = File::create(output_filename)?;
        output_file.write_all(buf.as_slice())?;
    }
    Ok(())
}
