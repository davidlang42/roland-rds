use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use roland::live_set::LiveSet;
use roland::tones::TONE_LIST;
use roland::tones::ToneNumber;
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
        let mut tone = 1;
        let tone_len = TONE_LIST.len() as u16 + 1;
        let mut file_num = 0;
        while tone < tone_len {
            let mut rds = RD300NX::from_bytes(bytes.clone().try_into().unwrap())?;
            for p in 0..rds.piano.len() {
                let mut names = Vec::new();
                let mut tn = ToneNumber(tone);
                names.push(tn.details().name);
                set_layer(&mut rds.piano[p], 0, tn);
                tone += 1;
                if tone >= tone_len {
                    rds.piano[p].common.name = make_name(names);
                    break;
                }
                tn = ToneNumber(tone);
                names.push(tn.details().name);
                set_layer(&mut rds.piano[p], 1, tn);
                tone += 1;
                if tone >= tone_len {
                    rds.piano[p].common.name = make_name(names);
                    break;
                }
                tn = ToneNumber(tone);
                names.push(tn.details().name);
                set_layer(&mut rds.piano[p], 2, tn);
                tone += 1;
                rds.piano[p].common.name = make_name(names);
                if tone >= tone_len {
                    break;
                }
            }
            for e in 0..rds.e_piano.len() {
                let mut names = Vec::new();
                let mut tn = ToneNumber(tone);
                names.push(tn.details().name);
                set_layer(&mut rds.e_piano[e], 0, tn);
                tone += 1;
                if tone >= tone_len {
                    rds.e_piano[e].common.name = make_name(names);
                    break;
                }
                tn = ToneNumber(tone);
                names.push(tn.details().name);
                set_layer(&mut rds.e_piano[e], 1, tn);
                tone += 1;
                if tone >= tone_len {
                    rds.e_piano[e].common.name = make_name(names);
                    break;
                }
                tn = ToneNumber(tone);
                names.push(tn.details().name);
                set_layer(&mut rds.e_piano[e], 2, tn);
                tone += 1;
                rds.e_piano[e].common.name = make_name(names);
                if tone >= tone_len {
                    break;
                }
            }
            for u in 0..rds.user_sets.len() {
                let mut names = Vec::new();
                let mut tn = ToneNumber(tone);
                names.push(tn.details().name);
                set_layer(&mut rds.user_sets[u], 0, tn);
                tone += 1;
                if tone >= tone_len {
                    rds.user_sets[u].common.name = make_name(names);
                    break;
                }
                tn = ToneNumber(tone);
                names.push(tn.details().name);
                set_layer(&mut rds.user_sets[u], 1, tn);
                tone += 1;
                if tone >= tone_len {
                    rds.user_sets[u].common.name = make_name(names);
                    break;
                }
                tn = ToneNumber(tone);
                names.push(tn.details().name);
                set_layer(&mut rds.user_sets[u], 2, tn);
                tone += 1;
                rds.user_sets[u].common.name = make_name(names);
                if tone >= tone_len {
                    break;
                }
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

fn set_layer(live_set: &mut LiveSet, l: usize, tone: ToneNumber) {
    live_set.common.split_switch_internal = true;
    live_set.layers[l].internal.enable = true;
    live_set.layers[l].internal.range_lower = match l {
        0 => PianoKey::A0,
        1 => PianoKey::C3,
        2 => PianoKey::C5,
        _ => panic!("Invalid layer")
    };
    live_set.layers[l].internal.range_upper = match l {
        0 => PianoKey::B2,
        1 => PianoKey::B4,
        2 => PianoKey::C8,
        _ => panic!("Invalid layer")
    };
    live_set.layers[l].internal.transpose = OffsetU8::<64>(match l {
        0 => 24,
        1 => 0,
        2 => -24,
        _ => panic!("Invalid layer")
    });
    live_set.layers[l].tone.tone_number = tone;
}

fn make_name(names: Vec<&str>) -> [char; 16] {
    // list words
    let mut words = Vec::new();
    for name in names {
        for word_with_dots in name.split(" ") {
            for word in word_with_dots.split(".") {
                words.push(word.to_owned());
            }
        }
        words.push("/".to_owned());
    }
    words.remove(words.len() - 1);
    // remove non-capital vowels
    while total_length(&words) > 16 {
        if !remove_vowels(&mut words, 16) {
            break;
        }
    }
    // remove middle lower case from longer words
    while total_length(&words) > 16 {
        let length = max_length(&words) - 1;
        if !remove_middle(&mut words, length, 16) {
            break;
        }
    }
    // pad with spaces
    let mut chars: Vec<char> = combine(&words).chars().collect();
    while chars.len() < 16 {
        chars.push(' ');
    }
    chars.try_into().unwrap()
}

fn total_length(words: &Vec<String>) -> usize {
    let mut sum = 0;
    for word in words {
        sum += word.len();
    }
    sum
}

fn max_length(words: &Vec<String>) -> usize {
    let mut max = 0;
    for word in words {
        if word.len() > max {
            max = word.len();
        }
    }
    max
}

fn combine(words: &Vec<String>) -> String {
    let mut s = String::new();
    for word in words {
        for c in word.chars() {
            s.push(c);
        }
    }
    s
}

fn remove_vowels(words: &mut Vec<String>, goal_length: usize) -> bool {
    const VOWELS: [char; 5] = ['a','e','i','o','u'];
    let mut changed = false;
    for i in 0..words.len() {
        if total_length(words) <= goal_length {
            break;
        } else {
            if let Some(pos) = words[i].chars().position(|c| VOWELS.iter().position(|v| *v == c).is_some()) {
                words[i].remove(pos);
                changed = true;
            }
        }
    }
    changed
}

fn remove_middle(words: &mut Vec<String>, min_length: usize, goal_length: usize) -> bool {
    let mut changed = false;
    for i in 0..words.len() {
        while words[i].len() > min_length && total_length(&words) > goal_length {
            if words[i].len() > 1 {
                let pos = words[i].len() / 2;
                words[i].remove(pos);
                //TODO avoid removing capitals
                changed = true;
            } else {
                break;
            }
        }
    }
    changed
}