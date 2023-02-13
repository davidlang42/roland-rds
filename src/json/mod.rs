use std::fmt::Debug;
use std::ops::AddAssign;
use std::path::PathBuf;
use std::fmt::Display;
use std::io;
use std::fs;

pub mod serialize_fromstr_display;
pub mod serialize_chars_as_string;
pub mod serialize_array_as_vec;

pub trait Json {
    fn to_json(&self) -> String;
    fn from_json(json: String) -> Self;
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

    pub fn from_collection<T: Json, F>(items: &[T], namer: F) -> Self where F: Fn(&T) -> Option<String> {
        let mut vec = Vec::new();
        let pad_length = digits(items.len());
        for (i, item) in items.iter().enumerate() {
            let name = match namer(item) {
                Some(s) => format!("{}-{}", pad(i, pad_length, '0'), alphanumeric(s)),
                None => pad(i, pad_length, '0')
            };
            vec.push((name, item.to_structured_json()))
        }
        Self::NestedCollection(vec)
    }

    pub fn load(path: PathBuf) -> Result<Self, io::Error> {
        if !path.exists() {
            panic!("Cannot load structured json, '{}' does not exist", path.display());
        }
        Ok(if path.is_dir() {
            let mut vec = Vec::new();
            for entry in path.read_dir().expect(&format!("Error reading directory '{}'", path.display())) {
                let e = entry?;
                vec.push((e.file_name().to_string_lossy().to_string(), Self::load(e.path())?));
            }
            vec.sort_by(|(a, _), (b, _)| a.cmp(b));
            Self::NestedCollection(vec)
        } else {
            Self::SingleJson(fs::read_to_string(path)?)
        })
    }

    pub fn extract(&mut self, name: &str) -> Self {
        match self {
            Self::SingleJson(_) => panic!("Cannot extract from StructuredJson::SingleJson"),
            Self::NestedCollection(vec) => {
                if let Some(i) = vec.iter().position(|(n, _)| n == name) {
                    vec.remove(i).1
                } else {
                    panic!("'{}' not found in StructuredJson::NestedCollection", name);
                }
            }
        }
    }

    pub fn done(self) {
        match self {
            Self::SingleJson(_) => panic!("Cannot call done on StructuredJson::SingleJson"),
            Self::NestedCollection(vec) => {
                if vec.len() > 0 {
                    let unused: Vec<String> = vec.into_iter().map(|(n, _)| n).collect();
                    panic!("Unused items in StructuredJson::NestedCollection {:?}", unused)
                }
            }
        }
    }

    pub fn to<T: Json>(self) -> T {
        T::from_structured_json(self)
    }

    pub fn to_vec<T: Json>(self) -> Vec<T> {
        match self {
            Self::SingleJson(_) => panic!("Cannot create collection from StructuredJson::SingleJson"),
            Self::NestedCollection(vec) => vec.into_iter().map(|(_, s)| T::from_structured_json(s)).collect()
        }
    }

    pub fn to_array<T: Json + Debug, const N: usize>(self) -> Box<[T; N]> {
        let vec = self.to_vec();
        let array: [T; N] = vec.try_into().unwrap();
        Box::new(array)
    }

    pub fn to_single_json(self) -> String {
        match self {
            Self::SingleJson(json) => json,
            Self::NestedCollection(_) => panic!("Cannot call to_single_json on StructuredJson::NestedCollection")
        }
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

fn alphanumeric(mut s: String) -> String {
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