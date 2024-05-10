use super::models::{BaseModel, BaseModelDto, DictObject, ToBaseModel};
use anyhow::{anyhow, Error};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_str, to_string, Deserializer};
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use tar::{Archive, Builder};

pub fn get_audio_file_from_compressed_archive(file_name: &str) -> Vec<u8> {
    let file = File::open("files/tone_archive.tar.zlib").unwrap();
    let decoder = ZlibDecoder::new(file);
    let mut archive = Archive::new(decoder);

    for file in archive.entries().unwrap() {
        let mut file = file.unwrap();
        if file.path().unwrap().to_str().unwrap() == file_name {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            return buffer;
        }
    }
    Vec::new()
}

pub fn get_pinyin_from_compressed_json(c: char) -> Option<String> {
    fn iter_json_array<T: DeserializeOwned, R: Read>(
        mut reader: R,
    ) -> impl Iterator<Item = Result<T, Error>> {
        fn read_skipping_ws(mut reader: impl Read) -> io::Result<u8> {
            loop {
                let mut byte = 0u8;
                reader.read_exact(std::slice::from_mut(&mut byte))?;
                if !byte.is_ascii_whitespace() {
                    return Ok(byte);
                }
            }
        }

        fn invalid_data(msg: &str) -> Error {
            anyhow!("Invalid data: {}", msg)
        }

        fn deserialize_single<T: DeserializeOwned, R: Read>(reader: R) -> Result<T, Error> {
            let next_obj = Deserializer::from_reader(reader).into_iter::<T>().next();
            next_obj.map_or_else(
                || Err(invalid_data("premature EOF")),
                |result| result.map_err(Into::into),
            )
        }

        fn yield_next_obj<T: DeserializeOwned, R: Read>(
            mut reader: R,
            at_start: &mut bool,
        ) -> Result<Option<T>, Error> {
            if *at_start {
                match read_skipping_ws(&mut reader)? {
                    b',' => deserialize_single(reader).map(Some),
                    b']' => Ok(None),
                    _ => Err(invalid_data("`,` or `]` not found")),
                }
            } else {
                *at_start = true;
                if read_skipping_ws(&mut reader)? == b'[' {
                    // read the next char to see if the array is empty
                    let peek = read_skipping_ws(&mut reader)?;
                    if peek == b']' {
                        Ok(None)
                    } else {
                        deserialize_single(io::Cursor::new([peek]).chain(reader)).map(Some)
                    }
                } else {
                    Err(invalid_data("`[` not found"))
                }
            }
        }

        let mut at_start = false;
        std::iter::from_fn(move || yield_next_obj(&mut reader, &mut at_start).transpose())
    }

    let file = File::open("files/dictionary.json.zlib").unwrap();
    let decoder = ZlibDecoder::new(file);
    let reader = BufReader::new(decoder);

    for value in iter_json_array::<DictObject, _>(reader) {
        match value {
            Ok(v) => {
                if v.traditional != c.to_string().into() {
                    continue;
                }
                return Some(v.pinyin.into());
            }
            Err(e) => {
                log::error!("Error: {}", e);
            }
        }
    }
    None
}

pub fn load_from_file_mut<T>(file_path: &str) -> Result<Vec<T>, Error>
where
    T: DeserializeOwned,
{
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let models: Vec<T> = from_str(&contents)?;
    Ok(models)
}

pub fn load_from_file<T>(file_path: &str) -> Result<Box<[T]>, Error>
where
    T: DeserializeOwned,
{
    Ok(load_from_file_mut(file_path)?.into_boxed_slice())
}

pub fn read_phrases() -> Result<Box<[BaseModel]>, Error> {
    let file_path = "files/phrases.json";
    let values = load_from_file::<BaseModelDto>(file_path)?;
    Ok(values
        .iter()
        .map(ToBaseModel::to_base_model)
        .collect::<Vec<_>>()
        .into_boxed_slice())
}
#[cfg(not(feature = "ignore_tools_arcive"))]
pub mod tools_archive {
    use super::{
        from_str, load_from_file, to_string, Archive, Builder, Compression, DictObject, Error,
        File, Read, Serialize, Write, ZlibEncoder,
    };

    pub fn _save_to_file<U, T>(models: T, file_path: &str) -> Result<(), Error>
    where
        U: Serialize,
        T: AsRef<[U]>,
    {
        let serialized = to_string(models.as_ref())?;
        let mut file = File::create(file_path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub(super) fn _read_dict() -> Result<Box<[DictObject]>, Error> {
        let file_path = "files/dictionary.json";
        load_from_file(file_path)
    }

    pub fn _read_compressed_dict() -> Result<Box<[DictObject]>, Error> {
        let file_path = "files/dictionary.json.zlib";
        let file = File::open(file_path)?;
        let mut decoder = flate2::read::ZlibDecoder::new(file);
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer)?;
        let models: Vec<DictObject> = from_str(&String::from_utf8(buffer)?)?;
        Ok(models.into_boxed_slice())
    }

    pub fn _make_tar_archive() {
        let file = File::create("files/tone_archive.tar").unwrap();
        let mut archive = Builder::new(file);

        for entry in std::fs::read_dir("files/tones2").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let file = &mut File::open(path.clone()).unwrap();
            archive
                .append_file(path.file_name().unwrap(), file)
                .unwrap();
        }
        archive.finish().unwrap();
    }

    pub fn _compress_tar_archive() {
        let mut file = File::open("files/tone_archive.tar").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&buffer).unwrap();
        let compressed_bytes = encoder.finish().unwrap();
        std::fs::write("files/tone_archive.tar.zlib", compressed_bytes).unwrap();
    }

    pub fn _make_compressed_tar_archive() {
        let file = File::create("files/tone_archive.tar.zlib").unwrap();
        let encoder = ZlibEncoder::new(file, Compression::best());
        let mut archive = Builder::new(encoder);

        for entry in std::fs::read_dir("files/tones2").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let file = &mut File::open(path.clone()).unwrap();
            archive
                .append_file(path.file_name().unwrap(), file)
                .unwrap();
        }
    }

    pub fn _decompress_tar_archive() {
        use flate2::read::ZlibDecoder;
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::open("files/tone_archive.tar.zlib").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let mut decoder = ZlibDecoder::new(&buffer[..]);
        let mut decompressed_bytes = Vec::new();
        decoder.read_to_end(&mut decompressed_bytes).unwrap();
        std::fs::write("files/tone_archive.tar", decompressed_bytes).unwrap();
    }

    pub fn _read_from_archive(file_name: &str) -> Vec<u8> {
        let file = File::open("files/tone_archive.tar").unwrap();
        let mut archive = Archive::new(file);
        for file in archive.entries().unwrap() {
            let mut file = file.unwrap();
            if file.path().unwrap().to_str().unwrap() == file_name {
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).unwrap();
                return buffer;
            }
        }
        Vec::new()
    }
}
