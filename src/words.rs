use super::models::{BaseModel, Pronouncation, Voice};
use super::utils::{get_random_word, StyledWrite};
use super::PHRASES;
use anyhow::Error;
use console::Term;
use crossbeam::channel::{bounded, Receiver};
use rand::random;
use std::collections::VecDeque;
use std::{cmp, thread};

pub struct Words {}

impl Words {
    fn start_basemodel_channel() -> Receiver<(BaseModel, Pronouncation)> {
        let channel_max_length = cmp::min(10, PHRASES.len());
        let (sender, receiver) = bounded(channel_max_length);

        thread::spawn(move || {
            let mut words: VecDeque<BaseModel> = VecDeque::with_capacity(channel_max_length);
            let mut channel_length: usize = 0;
            loop {
                let mut word = get_random_word(true);
                let voice = Voice::random();
                let tone: Pronouncation = Pronouncation::create_from(&mut word, &voice);

                if sender.len() < channel_length {
                    let pop_count = channel_length - sender.len();
                    for _ in 0..pop_count {
                        words.pop_front();
                    }
                }

                if !sender.is_full() && !words.contains(&word) {
                    sender.send((word.clone(), tone)).unwrap();
                    words.push_back(word);
                }

                channel_length = sender.len();

                // log::warn!(
                //     "VecDeque len: {}, channel len: {}",
                //     words.len(),
                //     sender.len()
                // );
            }
        });
        receiver
    }

    pub(super) fn run(terminal: &mut Term) -> Result<(), Error> {
        let receiver = Self::start_basemodel_channel();
        terminal.write_line("Words selected!\n")?;

        '_words: loop {
            let (mut word, pronounce) = receiver.recv().unwrap();
            let is_chinese = random::<bool>();

            let question = if is_chinese {
                word.question_chinese()
            } else {
                word.question_english()
            };

            terminal.write_question(&question)?;
            if is_chinese {
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
