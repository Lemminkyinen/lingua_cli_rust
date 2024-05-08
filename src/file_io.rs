use super::models::{BaseModel, DictObject};
use super::DICTIONARY;
use anyhow::Error;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string};
use std::fs::File;
use std::io::{Read, Write};

pub fn save_to_file<U, T>(models: T, file_path: &str) -> Result<(), Error>
where
    U: Serialize,
    T: AsRef<[U]>,
{
    let serialized = to_string(models.as_ref())?;
    let mut file = File::create(file_path)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

fn load_from_file_mut<T>(file_path: &str) -> Result<Vec<T>, Error>
where
    T: DeserializeOwned,
{
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let models: Vec<T> = from_str(&contents)?;
    Ok(models)
}

fn load_from_file<T>(file_path: &str) -> Result<Box<[T]>, Error>
where
    T: DeserializeOwned,
{
    Ok(load_from_file_mut(file_path)?.into_boxed_slice())
}

pub(super) fn read_dict() -> Result<Box<[DictObject]>, Error> {
    let file_path = "files/dictionary.json";
    load_from_file(file_path)
}

pub(super) fn read_compressed_dict() -> Result<Box<[DictObject]>, Error> {
    let file_path = "files/dictionary.json.zlib";
    let file = File::open(file_path)?;
    let mut decoder = flate2::read::ZlibDecoder::new(file);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;
    let models: Vec<DictObject> = from_str(&String::from_utf8(buffer)?)?;
    Ok(models.into_boxed_slice())
}

pub(super) fn read_phrases() -> Result<Box<[BaseModel]>, Error> {
    let file_path = "files/phrases.json";
    load_from_file(file_path)
}

fn _compress_dict() -> () {
    use flate2::write::ZlibEncoder;
    use flate2::Compression;
    use serde_json::to_vec;
    use std::io::prelude::*;

    let dictionary_bytes = to_vec(&DICTIONARY.to_vec()).unwrap();
    let mut e = ZlibEncoder::new(Vec::new(), Compression::best());
    e.write_all(&dictionary_bytes).unwrap();
    let compressed_bytes = e.finish().unwrap();
    println!("Dictionary size: {} bytes", dictionary_bytes.len());
    println!("Compressed size: {} bytes", compressed_bytes.len());
    std::fs::write("files/dictionary.json.zlib", compressed_bytes).unwrap();
}
