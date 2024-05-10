use std::io::{BufReader, Cursor};

use super::utils::string::{match_tone, normalize_char, normalize_word};
use crate::file_io::{get_audio_file_from_compressed_archive, get_pinyin_from_compressed_json};
use anyhow::Error;
use console::style;
use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BaseModelDto {
    pub(super) traditional: Box<[Box<str>]>,
    pub(super) simplified: Box<[Box<str>]>,
    pub(super) english: Box<[Box<str>]>,
    // extra
    pub(super) notes: Option<Box<[String]>>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct BaseModel {
    pub(super) traditional: Box<[Box<str>]>,
    pub(super) simplified: Box<[Box<str>]>,
    pub(super) english: Box<[Box<str>]>,
    // extra
    pub(super) notes: Option<Box<[String]>>,

    // hidden attributes
    pinyin_fetched: bool,
    pinyin: Option<Vec<String>>,
}
pub trait ToBaseModel {
    fn to_base_model(&self) -> BaseModel;
}

impl ToBaseModel for BaseModelDto {
    fn to_base_model(&self) -> BaseModel {
        BaseModel::new(
            self.traditional.clone(),
            self.simplified.clone(),
            self.english.clone(),
            self.notes.clone(),
        )
    }
}

impl BaseModel {
    pub(super) fn new(
        traditional: Box<[Box<str>]>,
        simplified: Box<[Box<str>]>,
        english: Box<[Box<str>]>,
        notes: Option<Box<[String]>>,
    ) -> Self {
        Self {
            traditional,
            simplified,
            english,
            notes,
            pinyin_fetched: false,
            pinyin: None,
        }
    }

    /// Fetches the pinyin for the traditional characters.
    /// Returns a vector of strings where each string is the pinyin for a word.
    ///
    /// Uses cached values if already fetched.
    pub fn pinyin(&mut self) -> Vec<String> {
        let time = std::time::Instant::now();
        let ret;
        if self.pinyin_fetched {
            ret = self.pinyin.as_ref().unwrap().clone();
        } else {
            ret = self
                .traditional
                .iter()
                .map(|w| {
                    w.chars()
                        .filter_map(get_pinyin_from_compressed_json)
                        .collect::<Vec<String>>()
                        .join(" ")
                })
                .collect::<Vec<_>>();
            self.pinyin = Some(ret.clone());
            self.pinyin_fetched = true;
        }

        log::info!("Elapsed time for fetching pinyins: {:?}", time.elapsed());
        ret
    }

    fn pinyin_alphabet(&mut self) -> Vec<String> {
        self.pinyin().iter().map(normalize_word).collect()
    }

    pub fn tones(&mut self) -> Vec<String> {
        self.pinyin()
            .iter()
            .map(|sentence| {
                let words = sentence.split_whitespace();
                let new_words: Vec<String> = words
                    .into_iter()
                    .map(|w| {
                        let mut tone = String::new();
                        let mut new_word = String::new();
                        w.chars().into_iter().for_each(|c| {
                            let tone_ = match_tone(c);
                            if [1, 2, 3, 4].contains(&tone_) {
                                tone = tone_.to_string();
                            }
                            new_word.push(normalize_char(c));
                        });
                        new_word.push_str(&tone);
                        new_word
                    })
                    .collect();
                new_words.join(" ")
            })
            .collect()
    }

    fn styled_pinyin(&mut self) -> String {
        self.pinyin()
            .iter()
            .map(|word| style(word).cyan().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn styled_english(&self) -> String {
        self.english
            .iter()
            .map(|word| style(word).cyan().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn styled_traditional(&self) -> String {
        self.traditional
            .iter()
            .map(|word| style(word).cyan().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn pinyin_alphabet_trimmed(&mut self) -> Vec<String> {
        self.pinyin_alphabet()
            .iter()
            .map(|w| w.to_lowercase().trim().replace(' ', ""))
            .collect()
    }

    fn english_trimmed(&self) -> Vec<String> {
        self.english
            .iter()
            .map(|w| w.trim().to_lowercase())
            .collect()
    }

    pub(super) fn question_chinese(&mut self) -> String {
        format!(
            "Here is a word in Chinese {}, {}. What is it in English?",
            self.styled_pinyin(),
            self.styled_traditional()
        )
    }

    pub(super) fn question_english(&self) -> String {
        format!(
            "Here is a word in English {}. What is it in Chinese?",
            self.styled_english()
        )
    }

    pub(super) fn handle_chinese_response(&self, english_res: &str) -> String {
        let txt = if self
            .english_trimmed()
            .contains(&english_res.trim().to_lowercase())
        {
            style("Correct! Well done!").green()
        } else {
            style("Wrong!").red()
        };

        format!(
            "{}. The English translation is: {}\n",
            txt,
            self.styled_english()
        )
    }

    pub(super) fn handle_english_response(&mut self, pinyin_res: &str) -> String {
        let txt = if self.pinyin_alphabet_trimmed().contains(
            &pinyin_res
                .trim()
                .to_lowercase()
                .replace(" ", "")
                .to_string(),
        ) {
            style("Correct! Well done!").green()
        } else {
            style("Wrong!").red()
        };

        format!(
            "{}. The Chinese translation is: {}, {}\n",
            txt,
            self.styled_pinyin(),
            self.styled_traditional()
        )
    }
}

pub fn _get_base_model() -> BaseModel {
    if random::<bool>() {
        BaseModel::new(
            Box::new(["你好".into()]),
            Box::new(["你好".into()]),
            Box::new(["hello".into()]),
            Some(Box::new(["This is a very common greeting.".to_string()])),
        )
    } else {
        BaseModel::new(
            Box::new(["我愛你".into()]),
            Box::new(["我爱你".into()]),
            Box::new(["I love you".into()]),
            Some(Box::new(["This is a very common phrase.".to_string()])),
        )
    }
}

pub enum Voice {
    MV1,
    MV2,
    MV3,
    FV1,
    FV2,
    FV3,
    Google,
}
impl Voice {
    pub fn random() -> Voice {
        match random::<u8>() % 6 {
            0 => Voice::MV1,
            1 => Voice::MV2,
            2 => Voice::MV3,
            3 => Voice::FV1,
            4 => Voice::FV2,
            _ => Voice::FV3,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Voice::MV1 => "MV1".to_string(),
            Voice::MV2 => "MV2".to_string(),
            Voice::MV3 => "MV3".to_string(),
            Voice::FV1 => "FV1".to_string(),
            Voice::FV2 => "FV2".to_string(),
            Voice::FV3 => "FV3".to_string(),
            Voice::Google => "Google".to_string(),
        }
    }
}

pub struct Pronouncation {
    bytes: Vec<Vec<Vec<u8>>>,
}

impl Pronouncation {
    pub fn create_from(word: &mut BaseModel, voice: &Voice) -> Self {
        let tones_str = word.tones();
        let voice_str = voice.to_string();
        let tones = tones_str
            .iter()
            .map(|phrase| {
                phrase
                    .split(' ')
                    .map(|word| {
                        get_audio_file_from_compressed_archive(&format!("{word}_{voice_str}.mp3"))
                    })
                    .collect::<Vec<Vec<u8>>>()
            })
            .collect::<Vec<Vec<Vec<u8>>>>();

        Self { bytes: tones }
    }

    pub fn play_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let (_stream, handle) = rodio::OutputStream::try_default()?;
        let sink = rodio::Sink::try_new(&handle)?;

        for phrase in &self.bytes {
            for word in phrase {
                let cursor = Cursor::new(word.clone());
                let source = rodio::Decoder::new(BufReader::new(cursor))?;
                sink.append(source);
            }
        }
        sink.sleep_until_end();
        Ok(())
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DictObject {
    pub(super) traditional: Box<str>,
    pub(super) simplified: Box<str>,
    pub(super) english: Box<str>,
    pub(super) pinyin: Box<str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_base_model() -> BaseModel {
        BaseModel::new(
            Box::new(["你好".into(), "我愛你".into()]),
            Box::new(["你好".into(), "我爱你".into()]),
            Box::new(["hello".into(), "I love you".into()]),
            Some(Box::new(["This is a very common greeting.".to_string()])),
        )
    }

    #[test]
    fn test_base_model_tones() {
        let mut model = get_base_model();
        assert_eq!(
            model.tones(),
            vec!["ni3 hao3".to_string(), "wo3 ai4 ni3".to_string()]
        );
    }

    #[test]
    fn test_get_pinying() {
        let mut model = BaseModel::new(
            Box::new(["請".into()]),
            Box::new(["請".into()]),
            Box::new(["please".into()]),
            None,
        );

        assert_eq!(
            model.pinyin(),
            std::iter::once(&"qǐng")
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
        );
    }
}
