use std::io::prelude::*;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

fn main() {
    // let mut start = 0;

    let mut input_file = File::open("/tmp/test-file").unwrap();
    let file_size = input_file.metadata().unwrap().len();
    println!("file size = {}", file_size);

    let chunk_size: usize = 10000;
    println!("chunk_size = {}", chunk_size);

    let chunk_num = (file_size / chunk_size as u64).try_into().unwrap();
    println!("chunks = {}", chunk_num);

    let remain = (file_size % chunk_size as u64).try_into().unwrap();
    println!("remaning bytes = {}", remain);

    for i in 0..chunk_num {
        let start = chunk_size * i;
        input_file
            .seek(SeekFrom::Start(start.try_into().unwrap()))
            .unwrap();
        let mut buf = vec![0; chunk_size];
        input_file.read_exact(&mut buf);

        let mut output_file = File::create(format!("/tmp/output-file{}", i)).unwrap();
        output_file.write_all(buf.as_slice()).unwrap();
    }

    if remain > 0 {
        let start = chunk_num * chunk_size;
        input_file
            .seek(SeekFrom::Start(start.try_into().unwrap()))
            .unwrap();
        let mut buf = vec![0; remain];
        input_file.read_exact(&mut buf);

        let mut output_file = File::create(format!("/tmp/output-file{}", chunk_num)).unwrap();
        output_file.write_all(buf.as_slice()).unwrap();
    }
}
