use std::fmt::Display;
use std::ops::AddAssign;
use std::path::PathBuf;
use std::fs;
use std::io;

#[derive(Debug)]
pub enum ParseError {
    IncorrectCheckSum {
        expected: Vec<u8>,
        found: Vec<u8>
    },
    InvalidCharacter(char)
}

pub trait Bytes<const N: usize> {
    const BYTE_SIZE: usize = N;

    fn to_bytes(&self) -> [u8; N];
    fn from_bytes(bytes: [u8; N]) -> Result<Self, ParseError> where Self: Sized;
    fn to_structured_json(&self) -> StructuredJson;
    fn from_structured_json(structured_json: StructuredJson) -> Self;
}

pub enum StructuredJson {
    SingleJson(String),
    NestedCollection(Vec<(String, StructuredJson)>)
}

impl StructuredJson {
    pub fn save(&self, path: PathBuf) -> Result<FileCount, io::Error> {
        if path.exists() {
            panic!("Cannot save structured json, '{}' already exists", path.display());
        }
        let mut count = FileCount::new();
        match self {
            Self::SingleJson(json) => {
                count.files += 1;
                fs::write(path, json)?;
            },
            Self::NestedCollection(vec) => {
                fs::create_dir(&path)?;
                count.folders += 1;
                for (name, structured_json) in vec {
                    let mut sub_path = path.clone();
                    sub_path.push(name);
                    let sub_count = structured_json.save(sub_path)?;
                    count += sub_count;
                }
            }
        }
        Ok(count)
    }

    pub fn from_collection<T: Bytes<N>, const N: usize, F>(items: &[T], namer: F) -> Self where F: Fn(&T) -> String {
        let mut vec = Vec::new();
        let pad_length = digits(items.len());
        for (i, item) in items.iter().enumerate() {
            vec.push((format!("{}-{}", pad(i, pad_length, '0'), clean_filename(namer(item))), item.to_structured_json()))
        }
        Self::NestedCollection(vec)
    }
}

pub struct FileCount {
    pub files: usize,
    pub folders: usize
}

impl FileCount {
    pub fn new() -> Self {
        FileCount {
            files: 0,
            folders: 0
        }
    }
}

impl AddAssign for FileCount {
    fn add_assign(&mut self, rhs: Self) {
        self.files += rhs.files;
        self.folders += rhs.folders;
    }
}

fn digits(mut number: usize) -> usize {
    let mut length = 0;
    while number != 0 {
        length += 1;
        number /= 10;
    }
    length
}

fn pad<T: Display>(item: T, length: usize, pad_with: char) -> String {
    let mut s = format!("{}", item);
    while s.len() < length {
        s.insert(0, pad_with);
    }
    s
}

fn clean_filename(mut s: String) -> String {
    let mut i = 0;
    while i < s.len() {
        let c = s.chars().nth(i).unwrap();
        if !c.is_alphanumeric() {
            s.remove(i);
        } else {
            i += 1;
        }
    }
    s
}