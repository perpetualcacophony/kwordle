use crate::letter::Letter;

use crate::letters::ParseLettersError;

pub enum ParseWordError {
    ParseLetters(ParseLettersError),
    NotInList { letters: Vec<Letter> },
}

impl From<ParseLettersError> for ParseWordError {
    fn from(value: ParseLettersError) -> Self {
        Self::ParseLetters(value)
    }
}
