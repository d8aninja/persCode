use std::io;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt");
    let buf = BufReader::new(file);
}