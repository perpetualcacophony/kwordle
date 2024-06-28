use crate::letter::letters::ParseLettersError;

#[derive(Debug, Clone)]
pub enum ParseWordsListError {
    ParseLetters(ParseLettersError),
    EmptyInput,
}

impl From<ParseLettersError> for ParseWordsListError {
    fn from(value: ParseLettersError) -> Self {
        Self::ParseLetters(value)
    }
}
