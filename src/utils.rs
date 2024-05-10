use super::{models::BaseModel, PHRASES};
use console::Term;
use rand::random;
use std::{fmt::Display, io::Write};

// Step 1: Define the trait
pub trait StyledWrite {
    fn write_styled_line<T: Display>(&mut self, styled_text: T) -> std::io::Result<()>;
    fn write_styled<T: Display>(&mut self, styled_text: T) -> Result<(), std::io::Error>;
    fn write_question<T: Display>(&mut self, styled_text: T) -> Result<(), std::io::Error>;
}

// Step 2: Implement the trait for the `Terminal` type
impl StyledWrite for Term {
    fn write_styled_line<T: Display>(&mut self, styled_text: T) -> std::io::Result<()> {
        // Convert the styled text to a string and write it
        self.write_line(&styled_text.to_string())
    }
    fn write_styled<T: Display>(&mut self, styled_text: T) -> Result<(), std::io::Error> {
        // Convert the styled text to a string and write it
        self.write_all(styled_text.to_string().as_bytes())
    }
    fn write_question<T: Display>(&mut self, styled_text: T) -> Result<(), std::io::Error> {
        // Convert the styled text to a string and write it
        self.write_all(format!("{styled_text}\n> ").as_bytes())
    }
}

pub fn get_random_word(load_attributes: bool) -> BaseModel {
    let mut base_model = PHRASES[random::<usize>() % PHRASES.len()].clone();
    if load_attributes {
        base_model.pinyin();
    }
    base_model
}

pub mod string {
    pub const fn match_tone(c: char) -> u8 {
        match c {
            'ā' | 'ē' | 'ī' | 'ō' | 'ū' => 1,
            'á' | 'é' | 'í' | 'ó' | 'ú' => 2,
            'ǎ' | 'ě' | 'ǐ' | 'ǒ' | 'ǔ' => 3,
            'à' | 'è' | 'ì' | 'ò' | 'ù' => 4,
            _ => 5,
        }
    }

    pub const fn normalize_char(c: char) -> char {
        match c {
            'ā' | 'á' | 'ǎ' | 'à' => 'a',
            'ē' | 'é' | 'ě' | 'è' => 'e',
            'ī' | 'í' | 'ǐ' | 'ì' => 'i',
            'ō' | 'ó' | 'ǒ' | 'ò' => 'o',
            'ū' | 'ú' | 'ǔ' | 'ù' | 'ǖ' | 'ǘ' | 'ǚ' | 'ǜ' | 'ü' => 'u',
            _ => c,
        }
    }

    pub fn normalize_word<S: AsRef<str>>(pinyin: S) -> String {
        pinyin.as_ref().chars().map(normalize_char).collect()
    }
}

// If word/phrase/sentence uses Chinese characters that do not have any tones
// then use Google translate to get the audio pronunciation
// https://translate.google.com/translate_tts?ie=UTF-8&q=了tl=zh-TW&client=tw-ob

// tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_tone() {
        assert_eq!(string::match_tone('ā'), 1);
        assert_eq!(string::match_tone('á'), 2);
        assert_eq!(string::match_tone('ǎ'), 3);
        assert_eq!(string::match_tone('à'), 4);
        assert_eq!(string::match_tone('a'), 5);
    }

    #[test]
    fn test_normalize_char() {
        assert_eq!(string::normalize_char('ā'), 'a');
        assert_eq!(string::normalize_char('á'), 'a');
        assert_eq!(string::normalize_char('ǎ'), 'a');
        assert_eq!(string::normalize_char('à'), 'a');
        assert_eq!(string::normalize_char('a'), 'a');
    }

    #[test]
    fn test_normalize_word() {
        assert_eq!(string::normalize_word("āáǎàa"), "aaaaa");
    }
}
