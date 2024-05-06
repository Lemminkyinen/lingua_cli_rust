use super::models::{BaseModel, DictObject};
use super::DICTIONARY;
use anyhow::Error;
use rayon::prelude::*;
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

pub(super) fn read_phrases() -> Result<Box<[BaseModel]>, Error> {
    let file_path = "files/phrases.json";
    load_from_file(file_path)
}

pub(super) fn get_traditional_pinyin(c: char) -> Option<String> {
    DICTIONARY
        .par_iter()
        .find_any(|d| d.traditional == c.to_string().into())
        .map(|d| d.pinyin.clone().into())
}
