use super::file_io::{get_audio_file_from_compressed_archive, get_pinyin_from_compressed_json};
use super::utils::string::{match_tone, normalize_char, normalize_word};
use console::style;
use rand::{random, Rng};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Cursor};

#[derive(Serialize, Deserialize)]
pub struct BaseModelDto {
    pub(super) traditional: Box<[Box<str>]>,
    pub(super) simplified: Box<[Box<str>]>,
    pub(super) english: Box<[Box<str>]>,
    // extra
    pub(super) notes: Option<Box<[String]>>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
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
        let ret;
        if self.pinyin_fetched {
            ret = self.pinyin.as_ref().unwrap().clone();
        } else {
            // Try to get pinyins for the whole sentence
            let mut pinyins = self
                .traditional
                .iter()
                .filter_map(get_pinyin_from_compressed_json)
                .peekable();

            ret = if pinyins.peek().is_none() {
                // If not found, then get pinyins for each individual char
                self.traditional
                    .iter()
                    .map(|w| {
                        w.chars()
                            .filter_map(|c| get_pinyin_from_compressed_json(&c))
                            .map(|p| p.to_lowercase())
                            .collect::<Vec<String>>()
                            .join(" ")
                    })
                    .collect::<Vec<_>>()
            } else {
                pinyins.collect()
            };

            self.pinyin = Some(ret.clone());
            self.pinyin_fetched = true;
        }
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
                        w.chars().for_each(|c| {
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

    pub fn has_normal_tones(&mut self) -> bool {
        self.tones().iter().any(|sentence| {
            sentence
                .split_whitespace()
                .any(|word| !word.chars().last().unwrap().is_numeric())
        })
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
        let txt = if self
            .pinyin_alphabet_trimmed()
            .contains(&pinyin_res.trim().to_lowercase().replace(' ', ""))
        {
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

#[derive(Debug, Clone)]
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
    pub fn random() -> Self {
        [
            Self::MV1,
            Self::MV2,
            Self::MV3,
            Self::FV1,
            Self::FV2,
            Self::FV3,
        ][rand::thread_rng().gen_range(0..6)]
        .clone()
    }
}

impl std::fmt::Display for Voice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::MV1 => "MV1",
            Self::MV2 => "MV2",
            Self::MV3 => "MV3",
            Self::FV1 => "FV1",
            Self::FV2 => "FV2",
            Self::FV3 => "FV3",
            Self::Google => "Google",
        }
        .to_string();
        write!(f, "{str}")
    }
}

#[derive(Debug, Clone)]
pub struct Pronouncation {
    bytes: Option<Vec<Vec<Vec<u8>>>>,
    google_bytes: Option<Vec<Vec<u8>>>,
}

impl Pronouncation {
    fn create_from_mp3(word: &mut BaseModel, voice: &Voice) -> Self {
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

        Self {
            bytes: Some(tones),
            google_bytes: None,
        }
    }
    fn create_from_google_translate(word: &BaseModel) -> Self {
        // If word/phrase/sentence uses Chinese characters that do not have any tones
        // then use Google translate to get the audio pronunciation
        // e.g. le, ma, ge,
        // https://translate.google.com/translate_tts?ie=UTF-8&q=了&tl=zh-TW&client=tw-ob

        let urls = word.traditional
            .iter()
            .map(|word| {
                format!("https://translate.google.com/translate_tts?ie=UTF-8&q={word}&tl=zh-TW&client=tw-ob")
            })
            .collect::<Vec<String>>();

        let results = urls
            .par_iter()
            .map(|url| {
                let response = reqwest::blocking::get(url).unwrap();
                response.bytes().unwrap().to_vec()
            })
            .collect::<Vec<Vec<u8>>>();

        Self {
            bytes: None,
            google_bytes: Some(results),
        }
    }

    pub fn create_from(word: &mut BaseModel, voice: &Voice) -> Self {
        if word.has_normal_tones() {
            return Self::create_from_google_translate(word);
        }
        Self::create_from_mp3(word, voice)
    }

    pub fn play_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let (_stream, handle) = rodio::OutputStream::try_default()?;
        let sink = rodio::Sink::try_new(&handle)?;

        // Play mp3 files
        if let Some(bytes) = &self.bytes {
            for phrase in bytes {
                for word in phrase {
                    let cursor = Cursor::new(word.clone());
                    let source = rodio::Decoder::new(BufReader::new(cursor))?;
                    sink.append(source);
                }
            }
            sink.sleep_until_end();
            return Ok(());
        }

        // Play google translate audio
        for phrase in self.google_bytes.as_ref().unwrap() {
            let cursor = Cursor::new(phrase.clone());
            let source = rodio::Decoder::new(BufReader::new(cursor))?;
            sink.append(source);
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
            Box::new(["你好".into(), "我愛你".into(), "你好了".into()]),
            Box::new(["你好".into(), "我爱你".into(), "你好了".into()]),
            Box::new(["hello".into(), "I love you".into(), "hello".into()]),
            Some(Box::new(["This is a very common greeting.".to_string()])),
        )
    }

    #[test]
    fn test_base_model_tones() {
        let mut model = get_base_model();
        assert_eq!(
            model.tones(),
            vec![
                "ni3 hao3".to_string(),
                "wo3 ai4 ni3".to_string(),
                "ni3 hao3 le".to_string()
            ]
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
