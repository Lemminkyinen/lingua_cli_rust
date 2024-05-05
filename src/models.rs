use console::style;
use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(super) struct BaseModel {
    pub(super) traditional: Box<[Box<str>]>,
    pub(super) simplified: Box<[Box<str>]>,
    pub(super) english: Box<[Box<str>]>,
    pub(super) pinyin: Box<[Box<str>]>,

    // extra
    pub(super) type_: Option<String>,
    pub(super) english_synonym: Option<Box<[String]>>,
    pub(super) chinese_synonym: Option<Box<[String]>>,
    pub(super) description: Option<Box<[String]>>,
    pub(super) tones: Option<Box<[String]>>,
    pub(super) notes: Option<Box<[String]>>,
}

impl BaseModel {
    fn pinyin_alphabet(&self) -> Vec<String> {
        self.pinyin
            .iter()
            .map(|w| {
                w.chars()
                    .map(|c| match c {
                        'ā' | 'á' | 'ǎ' | 'à' => 'a',
                        'ē' | 'é' | 'ě' | 'è' => 'e',
                        'ī' | 'í' | 'ǐ' | 'ì' => 'i',
                        'ō' | 'ó' | 'ǒ' | 'ò' => 'o',
                        'ū' | 'ú' | 'ǔ' | 'ù' => 'u',
                        'ǖ' | 'ǘ' | 'ǚ' | 'ǜ' | 'ü' => 'u',
                        _ => c,
                    })
                    .collect()
            })
            .collect()
    }

    fn styled_pinyin(&self) -> String {
        self.pinyin
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
            pinyin: Box::new(["nǐ hǎo".into()]),
            description: None,
            tones: None,
            type_: None,
            english_synonym: Some(Box::new(["hi".to_string()])),
            chinese_synonym: Some(Box::new(["你好".to_string()])),
            notes: Some(Box::new(["This is a very common greeting.".to_string()])),
        }
    } else {
        BaseModel {
            traditional: Box::new(["我愛你".into()]),
            simplified: Box::new(["我爱你".into()]),
            english: Box::new(["I love you".into()]),
            pinyin: Box::new(["wǒ ài nǐ".into()]),
            description: None,
            tones: None,
            type_: None,
            english_synonym: Some(Box::new(["I adore you".to_string()])),
            chinese_synonym: Some(Box::new(["我喜欢你".to_string()])),
            notes: Some(Box::new(["This is a very common phrase.".to_string()])),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct DictObject {
    pub(super) traditional: Box<str>,
    pub(super) simplified: Box<str>,
    pub(super) english: Box<str>,
    pub(super) pinyin: Box<str>,
}
