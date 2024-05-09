use super::utils::string_utils::{match_tone, normalize_char, normalize_word};
use crate::file_io::get_pinyin_from_compressed_json;
use console::style;
use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(super) struct BaseModel {
    pub(super) traditional: Box<[Box<str>]>,
    pub(super) simplified: Box<[Box<str>]>,
    pub(super) english: Box<[Box<str>]>,
    // extra
    pub(super) notes: Option<Box<[String]>>,
}

impl BaseModel {
    fn pinyin(&self) -> Vec<String> {
        self.traditional
            .iter()
            .map(|w| {
                w.chars()
                    .filter_map(get_pinyin_from_compressed_json)
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
    }

    fn pinyin_alphabet(&self) -> Vec<String> {
        self.pinyin().iter().map(normalize_word).collect()
    }

    fn tones(&self) -> Vec<String> {
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

    fn styled_pinyin(&self) -> String {
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

    fn pinyin_alphabet_trimmed(&self) -> Vec<String> {
        self.pinyin_alphabet()
            .iter()
            .map(|w| w.to_lowercase().trim().replace(" ", ""))
            .collect()
    }

    fn english_trimmed(&self) -> Vec<String> {
        self.english
            .iter()
            .map(|w| w.trim().to_lowercase())
            .collect()
    }

    pub(super) fn question_chinese(&self) -> String {
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

    pub(super) fn handle_english_response(&self, pinyin_res: &str) -> String {
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

pub(super) fn get_base_model() -> BaseModel {
    if random::<bool>() {
        BaseModel {
            traditional: Box::new(["你好".into()]),
            simplified: Box::new(["你好".into()]),
            english: Box::new(["hello".into()]),
            notes: Some(Box::new(["This is a very common greeting.".to_string()])),
        }
    } else {
        BaseModel {
            traditional: Box::new(["我愛你".into()]),
            simplified: Box::new(["我爱你".into()]),
            english: Box::new(["I love you".into()]),
            notes: Some(Box::new(["This is a very common phrase.".to_string()])),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(super) struct DictObject {
    pub(super) traditional: Box<str>,
    pub(super) simplified: Box<str>,
    pub(super) english: Box<str>,
    pub(super) pinyin: Box<str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_base_model() -> BaseModel {
        BaseModel {
            traditional: Box::new(["你好".into(), "我愛你".into()]),
            simplified: Box::new(["你好".into(), "我爱你".into()]),
            english: Box::new(["hello".into(), "I love you".into()]),
            notes: Some(Box::new(["This is a very common greeting.".to_string()])),
        }
    }

    #[test]
    fn test_base_model_tones() {
        let model = get_base_model();
        assert_eq!(
            model.tones(),
            vec!["ni3 hao3".to_string(), "wo3 ai4 ni3".to_string()]
        );
    }
}
