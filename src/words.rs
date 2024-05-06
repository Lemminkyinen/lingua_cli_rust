use super::models::get_base_model;
use super::utils::StyledWrite;
use anyhow::Error;
use console::Term;
use rand::random;

pub(super) struct Words {}

impl Words {
    pub(super) fn run(terminal: &mut Term) -> Result<(), Error> {
        terminal.write_line("Words selected!\n")?;
        '_words: loop {
            let word = get_base_model();
            let is_chinese = random::<bool>();

            let question = if is_chinese {
                word.question_chinese()
            } else {
                word.question_english()
            };

            terminal.write_question(&question)?;
            let input = terminal.read_line()?;

            let response = if is_chinese {
                word.handle_chinese_response(&input)
            } else {
                word.handle_english_response(&input)
            };
            terminal.write_line(&response)?;
        }
        Ok(())
    }
}
