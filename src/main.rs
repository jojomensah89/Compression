use flate2::{write::GzEncoder,Compression};
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;
use std::io::copy;
fn main() {
    file_compressor();
}


fn file_compressor () {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`")
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output,Compression::default());
    let start = Instant::now();

    copy(&mut input, &mut encoder).unwrap();

    let output = encoder.finish().unwrap();
    println!("Source len: {:?}", input.get_ref().metadata().unwrap().len());

    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());}