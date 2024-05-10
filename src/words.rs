use super::models::{BaseModel, Pronouncation, Voice};
use super::utils::{get_random_word, StyledWrite};
use anyhow::Error;
use console::Term;
use rand::random;
use std::{sync::mpsc, thread};

pub struct Words {}

impl Words {
    fn start_basemodel_pipe() -> mpsc::Receiver<(BaseModel, Pronouncation)> {
        let (base_model_tx, base_model_rx) = mpsc::sync_channel(10);
        thread::spawn(move || loop {
            let mut word = get_random_word(true);
            let voice = Voice::random();
            let tone: Pronouncation = Pronouncation::create_from(&mut word, &voice);
            base_model_tx.send((word, tone)).unwrap();
        });
        base_model_rx
    }

    pub(super) fn run(terminal: &mut Term) -> Result<(), Error> {
        let rx = Self::start_basemodel_pipe();
        terminal.write_line("Words selected!\n")?;

        '_words: loop {
            let (mut word, pronounce) = rx.recv().unwrap();
            let is_chinese = random::<bool>();

            let question = if is_chinese {
                pronounce.play_all().unwrap();
                word.question_chinese()
            } else {
                word.question_english()
            };

            terminal.write_question(&question)?;
            let input = terminal.read_line()?;

            let response = if is_chinese {
                word.handle_chinese_response(&input)
            } else {
                pronounce.play_all().unwrap();
                word.handle_english_response(&input)
            };
            terminal.write_line(&response)?;
        }
        Ok(())
    }
}
