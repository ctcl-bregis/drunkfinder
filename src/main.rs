use std::{fs::File, io::{prelude::*, BufReader}, path::Path,};
use crc32fast;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let testvalue = 5162020;

    let mut teststring: String;
    let mut checksum: u32;
    let mut bslice: &[u8];
    let lines = lines_from_file("wordlist");
    for x in &lines {
        for y in &lines {
            for z in &lines {
                teststring = format!("{}{}{}", x, y, z);
                bslice = teststring.as_bytes();
                checksum = crc32fast::hash(bslice);
                if checksum == testvalue {
                    println!("{} is a Drunk World seed", &teststring);
                }
            }
        }
    }
}

