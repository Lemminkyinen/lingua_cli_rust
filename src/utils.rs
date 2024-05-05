use console::Term;
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
