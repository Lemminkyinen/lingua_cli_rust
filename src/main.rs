#![warn(clippy::perf, clippy::pedantic, clippy::nursery)]

mod file_io;
mod models;
mod utils;
mod words;

use anyhow::Error;
use console::style;
use console::Term;
use lazy_static::lazy_static;
use models::BaseModel;
use std::io::Write;
use words::Words;

lazy_static! {
    static ref PHRASES: Box<[BaseModel]> = {
        match file_io::read_phrases() {
            Ok(phrases) => phrases,
            Err(e) => {
                log::error!("Error reading phrases: {}", e);
                std::process::exit(1);
            }
        }
    };
}

fn start_text() -> String {
    let welcome = style("Welcome to LinguaCLI!\n\n").bold();
    let options = style(
        "1. Words\n\
            2. Phrases\n\
            3. Sentences\n\
            4. Tones \n\
            5. Random\n\n",
    )
    .cyan();
    format!(
        "{welcome}\
        Please select what mode you would like to play:\n\
        {options}\
        Press Ctrl + C to exit\n\n> ",
    )
}

fn invalid_selection() -> String {
    format!(
        "{}\n> ",
        style("Invalid selection. Please try again.").red()
    )
}

fn not_implemented_yet() -> String {
    format!("{}", style("Not implemented yet.").red())
}

enum GameMode {
    Words = 1,
    Phrases = 2,
    Sentences = 3,
    Tones = 4,
    Random = 5,
}

impl GameMode {
    fn from_str(input: &str) -> Option<Self> {
        match input.to_lowercase().trim() {
            "1" | "words" => Some(Self::Words),
            "2" | "phrases" => Some(Self::Phrases),
            "3" | "sentences" => Some(Self::Sentences),
            "4" | "tones" => Some(Self::Tones),
            "5" | "random" => Some(Self::Random),
            _ => None,
        }
    }
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut terminal = Term::stdout();
    terminal.write_all(start_text().as_bytes())?;

    '_main: loop {
        let input = terminal.read_line()?;
        let Some(game_mode) = GameMode::from_str(&input) else {
            terminal.write_all(invalid_selection().as_bytes())?;
            continue '_main;
        };

        match game_mode {
            GameMode::Words => {
                Words::run(&mut terminal)?;
            }
            GameMode::Phrases => {
                terminal.write_line(&not_implemented_yet())?;
                terminal.write_all(invalid_selection().as_bytes())?;
            }
            GameMode::Sentences => {
                terminal.write_line(&not_implemented_yet())?;
                terminal.write_all(invalid_selection().as_bytes())?;
            }
            GameMode::Tones => {
                terminal.write_line(&not_implemented_yet())?;
                terminal.write_all(invalid_selection().as_bytes())?;
            }
            GameMode::Random => {
                terminal.write_line(&not_implemented_yet())?;
                terminal.write_all(invalid_selection().as_bytes())?;
            }
        }
    }
}
