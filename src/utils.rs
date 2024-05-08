use super::DICTIONARY;
use console::Term;
use rayon::prelude::*;
use std::{fmt::Display, io::Write};

// Step 1: Define the trait
pub(super) trait StyledWrite {
    fn write_styled_line<T: Display>(&mut self, styled_text: T) -> std::io::Result<()>;
    fn write_styled<T: Display>(&mut self, styled_text: T) -> std::io::Result<usize>;
    fn write_question<T: Display>(&mut self, styled_text: T) -> std::io::Result<usize>;
}

// Step 2: Implement the trait for the `Terminal` type
impl StyledWrite for Term {
    fn write_styled_line<T: Display>(&mut self, styled_text: T) -> std::io::Result<()> {
        // Convert the styled text to a string and write it
        self.write_line(&styled_text.to_string())
    }
    fn write_styled<T: Display>(&mut self, styled_text: T) -> std::io::Result<usize> {
        // Convert the styled text to a string and write it
        self.write(&styled_text.to_string().as_bytes())
    }
    fn write_question<T: Display>(&mut self, styled_text: T) -> std::io::Result<usize> {
        // Convert the styled text to a string and write it
        self.write(&format!("{}\n> ", styled_text).as_bytes())
    }
}

pub(super) fn get_traditional_pinyin(c: char) -> Option<String> {
    DICTIONARY
        .par_iter()
        .find_any(|d| d.traditional == c.to_string().into())
        .map(|d| d.pinyin.clone().into())
}
pub(super) mod string_utils {

    pub(crate) fn match_tone(c: char) -> u8 {
        match c {
            'ā' | 'ē' | 'ī' | 'ō' | 'ū' => 1,
            'á' | 'é' | 'í' | 'ó' | 'ú' => 2,
            'ǎ' | 'ě' | 'ǐ' | 'ǒ' | 'ǔ' => 3,
            'à' | 'è' | 'ì' | 'ò' | 'ù' => 4,
            _ => 5,
        }
    }

    pub(crate) fn normalize_char(c: char) -> char {
        match c {
            'ā' | 'á' | 'ǎ' | 'à' => 'a',
            'ē' | 'é' | 'ě' | 'è' => 'e',
            'ī' | 'í' | 'ǐ' | 'ì' => 'i',
            'ō' | 'ó' | 'ǒ' | 'ò' => 'o',
            'ū' | 'ú' | 'ǔ' | 'ù' => 'u',
            'ǖ' | 'ǘ' | 'ǚ' | 'ǜ' | 'ü' => 'u',
            _ => c,
        }
    }

    pub(crate) fn normalize_word<S: AsRef<str>>(pinyin: S) -> String {
        pinyin.as_ref().chars().map(normalize_char).collect()
    }
}
