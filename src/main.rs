use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;

use crate::bytes::Bytes;
use crate::roland::RD300NX;

mod roland;
mod bits;
mod bytes;

#[macro_use] extern crate serde_derive;

fn main() {
    let mut args = env::args();
    let cmd = args.next().unwrap();
    if let Some(verb) = args.next() {
        match verb.as_str() {
            "encode" => encode(args.next()),
            "decode" => decode(args.next()),
            "help" => help(&cmd),
            _ => {
                println!("Unrecognised command: {}", verb);
                help(&cmd)
            }
        }
    } else {
        help(&cmd)
    }
}

fn help(cmd: &str) {
    println!("Usage:");
    println!("  {} decode FILENAME -- decode RDS file and print JSON to std out", cmd);
    println!("  {} decode          -- decode RDS data from std in and print JSON to std out", cmd);
    println!("  {} encode FILENAME -- encode JSON file and print RDS data to std out", cmd);
    println!("  {} encode          -- encode JSON data from std in and print RDS data to std out", cmd);
}

fn decode(rds_path: Option<String>) {
    let (size, bytes) = read_data(rds_path);
    if size != RD300NX::BYTE_SIZE {
        println!("File should be {} bytes but found {}", RD300NX::BYTE_SIZE, size);
    } else {
        let rds = RD300NX::parse(bytes.try_into().unwrap()).expect("Error decoding RDS data");
        println!("{}", serde_json::to_string(&rds).expect("Error serializing JSON"));
    }
}

fn encode(json_path: Option<String>) {
    let (_, bytes) = read_data(json_path);
    let json: String = bytes.into_iter().map(|u| u as char).collect();
    let rds: RD300NX = serde_json::from_str(&json).expect("Error deserializing JSON");
    let mut stdout = io::stdout().lock();
    stdout.write_all(&rds.to_bytes()).expect("Error writing to std out");
    stdout.flush().expect("Error flushing std out");
}

fn read_data(path: Option<String>) -> (usize, Vec<u8>) {
    let mut bytes = Vec::new();
    let size = if let Some(filename) = path {
        let mut f = fs::File::options().read(true).open(&filename).expect(&format!("File could not be opened: {}", filename));
        f.read_to_end(&mut bytes).expect("Error reading file")
    } else {
        let stdin = io::stdin();
        let mut lock = stdin.lock();
        lock.read_to_end(&mut bytes).expect("Error reading std in")
    };
    (size, bytes)
}