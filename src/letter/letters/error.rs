use crate::letter::ParseLetterError;
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum ParseLettersError {
    ParseLetter(ParseLetterError),
    WrongLength { expected: usize, got: usize },
}

impl Error for ParseLettersError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParseLetter(err) => Some(err),
            _ => None,
        }
    }
}

impl Display for ParseLettersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseLetter(err) => write!(f, "failed to parse letter: {err}"),
            Self::WrongLength { expected, got } => {
                write!(f, "input is {got} letters long, expected {expected}")
            }
        }
    }
}

impl From<ParseLetterError> for ParseLettersError {
    fn from(value: ParseLetterError) -> Self {
        Self::ParseLetter(value)
    }
}
