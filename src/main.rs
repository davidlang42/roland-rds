use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use roland::tones::TONE_LIST;
use roland::tones::ToneNumber;
use roland::types::notes::MidiNote;
use roland::types::notes::PianoKey;
use roland::types::numeric::OffsetU8;

use crate::bytes::Bytes;
use crate::json::{Json, StructuredJson};
use crate::roland::rd300nx::RD300NX;

mod roland;
mod bytes;
mod json;

#[macro_use] extern crate serde_derive;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");
    let mut args = env::args();
    let cmd = args.next().unwrap();
    if let Some(verb) = args.next() {
        match verb.as_str() {
            "encode" => encode(
                optional(args.next().ok_or("The 2nd argument should be the FILENAME for the input JSON file (or '-' for STDIN)")?),
                optional(args.next().ok_or("The 3rd argument should be the FILENAME for the output RDS file (or '-' for STDOUT)")?)
            )?,
            "decode" => decode(
                optional(args.next().ok_or("The 2nd argument should be the FILENAME for the input RDS file (or '-' for STDIN)")?),
                optional(args.next().ok_or("The 3rd argument should be the FILENAME for the output JSON file (or '-' for STDOUT)")?)
            )?,
            "split" => split(
                optional(args.next().ok_or("The 2nd argument should be the FILENAME for the input JSON file (or '-' for STDIN)")?),
                args.next().ok_or("The 3rd argument should be the FOLDER for the JSON file to be split into (and must not exist)")?
            )?,
            "merge" => merge(
                args.next().ok_or("The 2nd argument should be the FOLDER containing the JSON data to combine")?,
                optional(args.next().ok_or("The 3rd argument should be the FILENAME for the output JSON file (or '-' for STDOUT)")?),
            )?,
            "tone_test" => tone_test(
                args.next().ok_or("The 2nd argument should be the BASE FILE")?,
                args.next().ok_or("The 3rd argument should be the FOLDER for the output RDS files")?,
            )?,
            "help" => help(&cmd),
            _ => {
                println!("The 1st argument did not contain a valid command: {}", verb);
                help(&cmd)
            }
        }
    } else {
        help(&cmd)
    }
    Ok(())
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
    println!("  - folders cannot be STDIN/STDOUT and must be specified");
    println!("  - STDIN/STDOUT does not support binary data on Windows");
}

fn decode(input_rds: Option<String>, output_json: Option<String>) -> Result<(), Box<dyn Error>> {
    let (size, bytes) = read_data(&input_rds)?;
    if size != RD300NX::BYTE_SIZE {
        Err(format!("File should be {} bytes but found {}", RD300NX::BYTE_SIZE, size).into())
    } else {
        let rds = RD300NX::from_bytes(bytes.try_into().unwrap())?;
        write_json(&output_json, rds.to_json())?;
        if let Some(file) = &output_json {
            println!("Decoded RDS data into '{}'", file);
        }
        Ok(())
    }
}

fn encode(input_json: Option<String>, output_rds: Option<String>) -> Result<(), Box<dyn Error>> {
    let rds = read_json(&input_json)?;
    write_data(&output_rds, &*rds.to_bytes()?)?;
    if let Some(file) = &output_rds {
        println!("Encoded RDS data into '{}'", file);
    }
    Ok(())
}

fn split(input_json: Option<String>, output_folder: String) -> Result<(), Box<dyn Error>> {
    let rds = read_json(&input_json)?;
    let structure = rds.to_structured_json();
    let count = structure.save(PathBuf::from(&output_folder))?;
    println!("Split JSON into {} files in '{}'", count.files, output_folder);
    Ok(())
}

fn merge(input_folder: String, output_json: Option<String>) -> Result<(), Box<dyn Error>> {
    let structure = StructuredJson::load(PathBuf::from(&input_folder))?;
    let rds = RD300NX::from_structured_json(structure)?;
    write_json(&output_json, rds.to_json())?;
    if let Some(file) = &output_json {
        println!("Merged JSON into '{}'", file);
    }
    Ok(())
}

fn read_json(path: &Option<String>) -> Result<Box<RD300NX>, Box<dyn Error>> {
    let (_, bytes) = read_data(path)?;
    let text: String = bytes.into_iter().map(|u| u as char).collect();
    let rds = RD300NX::from_json(text)?;
    Ok(Box::new(rds))
}

fn write_json(path: &Option<String>, json: String) -> Result<(), io::Error> {
    let bytes: Vec<u8> = json.chars().map(|c| c as u8).collect();
    write_data(path, bytes.as_slice())
}

fn read_data(path: &Option<String>) -> Result<(usize, Vec<u8>), io::Error> {
    let mut bytes = Vec::new();
    let size = if let Some(filename) = path {
        let mut f = fs::File::options().read(true).open(&filename)?;
        f.read_to_end(&mut bytes)?
    } else {
        let stdin = io::stdin();
        let mut lock = stdin.lock();
        lock.read_to_end(&mut bytes)?
    };
    Ok((size, bytes))
}

fn write_data(path: &Option<String>, bytes: &[u8]) -> Result<(), io::Error> {
    if let Some(filename) = path {
        let mut f = fs::File::options().create(true).write(true).truncate(true).open(&filename)?;
        f.write_all(&bytes)?;
        f.flush()?;
    } else {
        let mut stdout = io::stdout().lock();
        stdout.write_all(&bytes)?;
        stdout.flush()?;
    }
    Ok(())
}

fn tone_test(base_path: String, output_folder: String) -> Result<(), Box<dyn Error>> {
    let (size, bytes) = read_data(&Some(base_path))?;
    if size != RD300NX::BYTE_SIZE {
        Err(format!("File should be {} bytes but found {}", RD300NX::BYTE_SIZE, size).into())
    } else {
        let mut tone = 0;
        let tone_len = TONE_LIST.len() as u16;
        let mut file_num = 0;
        while tone < tone_len {
            let mut rds = RD300NX::from_bytes(bytes.clone().try_into().unwrap())?;
            for u in 0..rds.user_sets.len() {
                let mut names = Vec::new();
                rds.user_sets[u].layers[0].internal.enable = true;
                rds.user_sets[u].layers[0].internal.range_lower = PianoKey::A0;
                rds.user_sets[u].layers[0].internal.range_upper = PianoKey::C3;
                rds.user_sets[u].layers[0].internal.transpose = OffsetU8::<64>(-24);
                rds.user_sets[u].layers[0].tone.tone_number = ToneNumber(tone);
                names.push(rds.user_sets[u].layers[0].tone.tone_number.details().name);
                tone += 1;
                if tone >= tone_len {
                    rds.user_sets[u].common.name = make_name(names);
                    break;
                }
                rds.user_sets[u].layers[1].internal.enable = true;
                rds.user_sets[u].layers[1].internal.range_lower = PianoKey::A0;
                rds.user_sets[u].layers[1].internal.range_upper = PianoKey::C3;
                rds.user_sets[u].layers[1].internal.transpose = OffsetU8::<64>(-24);
                rds.user_sets[u].layers[1].tone.tone_number = ToneNumber(tone);
                names.push(rds.user_sets[u].layers[1].tone.tone_number.details().name);
                tone += 1;
                if tone >= tone_len {
                    rds.user_sets[u].common.name = make_name(names);
                    break;
                }
                rds.user_sets[u].layers[2].internal.enable = true;
                rds.user_sets[u].layers[2].internal.range_lower = PianoKey::A0;
                rds.user_sets[u].layers[2].internal.range_upper = PianoKey::C3;
                rds.user_sets[u].layers[2].internal.transpose = OffsetU8::<64>(-24);
                rds.user_sets[u].layers[2].tone.tone_number = ToneNumber(tone);
                names.push(rds.user_sets[u].layers[2].tone.tone_number.details().name);
                tone += 1;
                rds.user_sets[u].common.name = make_name(names);
            }
            let output_json = format!("{}/tones{}.json", output_folder, file_num);
            write_json(&Some(output_json), rds.to_json())?;
            let output_rds = format!("{}/TONES{}.RDS", output_folder, file_num);
            write_data(&Some(output_rds), &*rds.to_bytes()?)?;
            file_num += 1;
        }
        println!("Saved all tones into {} files", file_num);
        Ok(())
    }
}

fn make_name(names: Vec<&str>) -> [char; 16] {
    let str_names: Vec<String> = names.iter().map(|s| format!("{}", s)).collect();
    let full_name = str_names.join("/");
    let mut chars: Vec<char> = full_name.chars().collect();
    // make it shorter
    if chars.len() > 16 {
        chars = chars.into_iter().filter(|c| c.is_alphanumeric()).collect();
    }
    //TODO more shortening
    // pad with spaces
    while chars.len() < 16 {
        chars.push(' ');
    }
    chars.try_into().unwrap()
}