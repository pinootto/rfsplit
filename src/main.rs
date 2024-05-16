use std::io::prelude::*;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

fn main() {
    let mut start = 0;

    let mut input_file = File::open("/tmp/test-file").unwrap();
    let file_size = input_file.metadata().unwrap().len();
    println!("file size = {}", file_size);

    let chunk_size: usize = 10000;
    println!("chunk_size = {}", chunk_size);

    let chunk_num = file_size / chunk_size as u64;
    println!("chunks = {}", chunk_num);

    let remain = file_size % chunk_size as u64;
    println!("remaning bytes = {}", remain);

    input_file.seek(SeekFrom::Start(start)).unwrap();
    let mut buf = vec![0; chunk_size];
    input_file.read_exact(&mut buf);

    let mut output_file = File::create("/tmp/output-file").unwrap();
    output_file.write_all(buf.as_slice()).unwrap();
}
