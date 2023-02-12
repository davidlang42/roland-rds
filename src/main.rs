use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use crate::bytes::Bytes;
use crate::bytes::StructuredJson;
use crate::roland::rd300nx::RD300NX;

mod roland;
mod bits;
mod bytes;
mod json;

#[macro_use] extern crate serde_derive;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut args = env::args();
    let cmd = args.next().unwrap();
    if let Some(verb) = args.next() {
        match verb.as_str() {
            "encode" => encode(args.next()),
            "decode" => decode(args.next()),
            "split" => split(args.next().expect("The 2nd argument should be the FILENAME to split or the FOLDER to output to"), args.next()),
            "merge" => merge(args.next().expect("The 2nd argument should be the FOLDER containing the JSON data to combine")),
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
    println!("roland-rds (v{})", VERSION);
    println!("Usage:");
    println!("  {} decode FILENAME       -- decode RDS file and print JSON to std out", cmd);
    println!("  {} decode                -- decode RDS data from std in and print JSON to std out", cmd);
    println!("  {} encode FILENAME       -- encode JSON file and print RDS data to std out", cmd);
    println!("  {} encode                -- encode JSON data from std in and print RDS data to std out", cmd);
    println!("  {} split FILENAME FOLDER -- split JSON file into a folder structure of nested JSON files", cmd);
    println!("  {} split FOLDER          -- split JSON data fro std in into a folder structure of nested JSON files", cmd);
    println!("  {} merge FOLDER          -- merge folder structure of nested JSON files and print the combined JSON data to std out", cmd);
}

fn decode(rds_path: Option<String>) {
    let (size, bytes) = read_data(rds_path);
    if size != RD300NX::BYTE_SIZE {
        println!("File should be {} bytes but found {}", RD300NX::BYTE_SIZE, size);
    } else {
        let rds = RD300NX::from_bytes(bytes.try_into().unwrap()).expect("Error decoding RDS data");
        println!("{}", rds.to_json());
    }
}

fn encode(json_path: Option<String>) {
    let rds = load_json(json_path);
    let mut stdout = io::stdout().lock();
    stdout.write_all(&*rds.to_bytes()).expect("Error writing to std out");
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

fn load_json(path: Option<String>) -> Box<RD300NX> {
    let (_, bytes) = read_data(path);
    let text: String = bytes.into_iter().map(|u| u as char).collect();
    let rds = RD300NX::from_json(text);
    Box::new(rds)
}

fn split(arg1: String, arg2: Option<String>) {
    let (filename, folder) = if let Some(folder) = arg2 {
        (Some(arg1), folder)
    } else {
        (None, arg1)
    };
    let rds = load_json(filename);
    let structure = rds.to_structured_json();
    let count = structure.save(PathBuf::from(&folder)).expect("Error saving structured JSON");
    println!("Split JSON into {} files in '{}'", count.files, folder);
}

fn merge(folder: String) {
    let structure = StructuredJson::load(PathBuf::from(&folder)).expect("Error loading structured JSON");
    let rds = RD300NX::from_structured_json(structure);
    println!("{}", rds.to_json());
}