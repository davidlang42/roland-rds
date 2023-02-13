use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use crate::bytes::Bytes;
use crate::json::{Json, StructuredJson};
use crate::roland::rd300nx::RD300NX;

mod roland;
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
            "encode" => encode(
                optional(args.next().expect("The 2nd argument should be the FILENAME for the input JSON file (or '-' for STDIN)")),
                optional(args.next().expect("The 3rd argument should be the FILENAME for the output RDS file (or '-' for STDOUT)"))
            ),
            "decode" => decode(
                optional(args.next().expect("The 2nd argument should be the FILENAME for the input RDS file (or '-' for STDIN)")),
                optional(args.next().expect("The 3rd argument should be the FILENAME for the output JSON file (or '-' for STDOUT)"))
            ),
            "split" => split(
                optional(args.next().expect("The 2nd argument should be the FILENAME for the input JSON file (or '-' for STDIN)")),
                args.next().expect("The 3rd argument should be the FOLDER for the JSON file to be split into (and must not exist)")
            ),
            "merge" => merge(
                args.next().expect("The 2nd argument should be the FOLDER containing the JSON data to combine"),
                optional(args.next().expect("The 3rd argument should be the FILENAME for the output JSON file (or '-' for STDOUT)")),
            ),
            "help" => help(&cmd),
            _ => {
                println!("The 1st argument did not contain a valid command: {}", verb);
                help(&cmd)
            }
        }
    } else {
        help(&cmd)
    }
}

fn optional(arg: String) -> Option<String> {
    if arg == "-" {
        None
    } else {
        Some(arg)
    }
}

fn help(cmd: &str) {
    println!("roland-rds (v{})", VERSION);
    println!("Usage:");
    println!("  {} decode INPUT.RDS OUTPUT.JSON     -- read RDS file and write to JSON file", cmd);
    println!("  {} encode INPUT.JSON OUTPUT.RDS     -- read JSON file and write to RDS file", cmd);
    println!("  {} split INPUT.JSON OUTPUT_FOLDER   -- split JSON file into a folder structure of nested JSON files", cmd);
    println!("  {} merge INPUT_FOLDER OUTPUT.JSON   -- merge folder structure of nested JSON files into a JSON file", cmd);
    println!("In all instances, '-' can be used as a file argument to indicate STDIN or STDOUT, however");
    println!("  - folders cannot be STDIN/STDOUT and must always be specified");
    println!("  - STDIN/STDOUT do not support binary data on Windows");
}

fn decode(input_rds: Option<String>, output_json: Option<String>) {
    let (size, bytes) = read_data(&input_rds);
    if size != RD300NX::BYTE_SIZE {
        println!("File should be {} bytes but found {}", RD300NX::BYTE_SIZE, size);
    } else {
        let rds = RD300NX::from_bytes(bytes.try_into().unwrap()).expect("Error decoding RDS data");
        write_json(&output_json, rds.to_json());
        if let Some(file) = &output_json {
            println!("Decoded RDS data into '{}'", file);
        }
    }
}

fn encode(input_json: Option<String>, output_rds: Option<String>) {
    let rds = read_json(&input_json);
    write_data(&output_rds, rds.to_bytes().as_slice());
    if let Some(file) = &output_rds {
        println!("Encoded RDS data into '{}'", file);
    }
}

fn split(input_json: Option<String>, output_folder: String) {
    let rds = read_json(&input_json);
    let structure = rds.to_structured_json();
    let count = structure.save(PathBuf::from(&output_folder)).expect("Error saving structured JSON");
    println!("Split JSON into {} files in '{}'", count.files, output_folder);
}

fn merge(input_folder: String, output_json: Option<String>) {
    let structure = StructuredJson::load(PathBuf::from(&input_folder)).expect("Error loading structured JSON");
    let rds = RD300NX::from_structured_json(structure);
    write_json(&output_json, rds.to_json());
    if let Some(file) = &output_json {
        println!("Merged JSON into '{}'", file);
    }
}

fn read_json(path: &Option<String>) -> Box<RD300NX> {
    let (_, bytes) = read_data(path);
    let text: String = bytes.into_iter().map(|u| u as char).collect();
    let rds = RD300NX::from_json(text);
    Box::new(rds)
}

fn write_json(path: &Option<String>, json: String) {
    let bytes: Vec<u8> = json.chars().map(|c| c as u8).collect();
    write_data(path, bytes.as_slice())
}

fn read_data(path: &Option<String>) -> (usize, Vec<u8>) {
    let mut bytes = Vec::new();
    let size = if let Some(filename) = path {
        let mut f = fs::File::options().read(true).open(&filename).expect(&format!("File could not be opened: {}", filename));
        f.read_to_end(&mut bytes).expect("Error reading file")
    } else {
        let stdin = io::stdin();
        let mut lock = stdin.lock();
        lock.read_to_end(&mut bytes).expect("Error reading STDIN")
    };
    (size, bytes)
}

fn write_data(path: &Option<String>, bytes: &[u8]) {
    if let Some(filename) = path {
        let mut f = fs::File::options().write(true).open(&filename).expect(&format!("File could not be opened: {}", filename));
        f.write_all(&bytes).expect("Error writing file");
        f.flush().expect("Error flushing file");
    } else {
        let mut stdout = io::stdout().lock();
        stdout.write_all(&bytes).expect("Error writing to STDOUT");
        stdout.flush().expect("Error flushing STDOUT");
    }
}
