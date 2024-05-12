use super::models::{BaseModel, Pronouncation, Voice};
use super::utils::{get_random_base_model, StyledWrite};
use super::PHRASES;
use anyhow::Error;
use console::Term;
use crossbeam::channel::{bounded, Receiver};
use rand::random;
use std::collections::VecDeque;
use std::{cmp, thread};

pub enum Mode {
    Words,
    Phrases,
    Sentences,
    Tones,
    Random,
}

impl Mode {
    pub fn get_json_file(&self) -> &'static [BaseModel] {
        match self {
            Mode::Words => &super::WORDS,
            Mode::Phrases => &super::PHRASES,
            Mode::Sentences => &super::SENTENCES,
            Mode::Tones => &super::PHRASES,
            Mode::Random => &super::PHRASES,
        }
    }
}

pub struct Language {}

impl Language {
    fn start_basemodel_channel(mode: Mode) -> Receiver<(BaseModel, Pronouncation)> {
        let channel_max_length = cmp::min(10, PHRASES.len());
        let (sender, receiver) = bounded(channel_max_length);

        thread::spawn(move || {
            let mut base_models: VecDeque<BaseModel> = VecDeque::with_capacity(channel_max_length);
            loop {
                let mut base_model = get_random_base_model(&mode, true);

                while sender.is_full() {
                    thread::sleep(std::time::Duration::from_millis(1200));
                }

                if sender.len() < base_models.len() {
                    let pop_count = base_models.len() - sender.len();
                    for _ in 0..pop_count {
                        base_models.pop_front();
                    }
                }

                if !sender.is_full() && !base_models.contains(&base_model) {
                    let voice = Voice::random();
                    let tone: Pronouncation = Pronouncation::create_from(&mut base_model, &voice);
                    sender.send((base_model.clone(), tone)).unwrap();
                    base_models.push_back(base_model);
                }
            }
        });
        receiver
    }

    pub(super) fn run(terminal: &mut Term, mode: Mode) -> Result<(), Error> {
        let receiver = Self::start_basemodel_channel(mode);
        terminal.write_line("Words selected!\n")?;
        let mut last_round_chinese: bool;

        '_words: loop {
            let (mut word, pronounce) = receiver.recv().unwrap();
            let is_chinese = random::<bool>();

            let question = if is_chinese {
                last_round_chinese = true;
                word.question_chinese()
            } else {
                last_round_chinese = false;
                word.question_english()
            };

            terminal.write_question(&question)?;
            if is_chinese {
                if last_round_chinese {
                    thread::sleep(std::time::Duration::from_millis(720));
                }
                pronounce.play_all().unwrap();
            }

            let input = terminal.read_line()?;
            let response = if is_chinese {
                word.handle_chinese_response(&input)
            } else {
                word.handle_english_response(&input)
            };

            terminal.write_line(&response)?;
            if !is_chinese {
                pronounce.play_all().unwrap();
            }
        }
        Ok(())
    }
}
