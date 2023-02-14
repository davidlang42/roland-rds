use std::error::Error;
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
    fn from_json(json: String) -> Result<Self, serde_json::Error> where Self: Sized;
    fn to_structured_json(&self) -> StructuredJson;
    fn from_structured_json(structured_json: StructuredJson) -> Result<Self, StructuredJsonError> where Self: Sized;
}

#[derive(Debug)]
pub enum StructuredJsonError {
    JsonError(serde_json::Error),
    NodeNotFound(String),
    ExpectedFolderButFoundFile,
    ExpectedFileButFoundFolder,
    UnusedNodes(Vec<String>)
}

impl From<serde_json::Error> for StructuredJsonError {
    fn from(value: serde_json::Error) -> Self {
        StructuredJsonError::JsonError(value)
    }
}

impl Error for StructuredJsonError {}

impl Display for StructuredJsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub enum StructuredJson {
    SingleJson(String),
    NestedCollection(Vec<(String, StructuredJson)>)
}

impl StructuredJson {
    pub fn save(&self, path: PathBuf) -> Result<FileCount, io::Error> {
        if path.exists() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, format!("Cannot save structured json, '{}' already exists", path.display())));
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
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("Cannot load structured json, '{}' does not exist", path.display())));
        }
        Ok(if path.is_dir() {
            let mut vec = Vec::new();
            for entry in path.read_dir()? {
                let e = entry?;
                vec.push((e.file_name().to_string_lossy().to_string(), Self::load(e.path())?));
            }
            vec.sort_by(|(a, _), (b, _)| a.cmp(b));
            Self::NestedCollection(vec)
        } else {
            Self::SingleJson(fs::read_to_string(path)?)
        })
    }

    pub fn extract(&mut self, name: &str) -> Result<Self, StructuredJsonError> {
        match self {
            Self::SingleJson(_) => Err(StructuredJsonError::ExpectedFolderButFoundFile),
            Self::NestedCollection(vec) => {
                if let Some(i) = vec.iter().position(|(n, _)| n == name) {
                    Ok(vec.remove(i).1)
                } else {
                    Err(StructuredJsonError::NodeNotFound(name.to_owned()))
                }
            }
        }
    }

    pub fn done(self) -> Result<(), StructuredJsonError> {
        match self {
            Self::SingleJson(_) => Err(StructuredJsonError::ExpectedFolderButFoundFile),
            Self::NestedCollection(vec) => {
                if vec.len() > 0 {
                    let unused: Vec<String> = vec.into_iter().map(|(n, _)| n).collect();
                    Err(StructuredJsonError::UnusedNodes(unused))
                } else {
                    Ok(())
                }
            }
        }
    }

    pub fn to<T: Json>(self) -> Result<T, StructuredJsonError> {
        T::from_structured_json(self)
    }

    pub fn to_vec<T: Json>(self) -> Result<Vec<T>, StructuredJsonError> {
        match self {
            Self::SingleJson(_) => Err(StructuredJsonError::ExpectedFolderButFoundFile),
            Self::NestedCollection(vec) => vec.into_iter().map(|(_, s)| T::from_structured_json(s)).collect()
        }
    }

    pub fn to_array<T: Json + Debug, const N: usize>(self) -> Result<Box<[T; N]>, StructuredJsonError> {
        let vec = self.to_vec()?;
        let array: [T; N] = vec.try_into().unwrap();
        Ok(Box::new(array))
    }

    pub fn to_single_json(self) -> Result<String, StructuredJsonError> {
        match self {
            Self::SingleJson(json) => Ok(json),
            Self::NestedCollection(_) => Err(StructuredJsonError::ExpectedFileButFoundFolder)
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