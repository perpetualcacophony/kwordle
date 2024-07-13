use crate::letter::ParseLetterError;
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum ParseLettersError {
    ParseLetter(ParseLetterError),
    WrongLength(crate::array::LengthError),
}

impl Error for ParseLettersError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParseLetter(err) => Some(err),
            Self::WrongLength(err) => Some(err),
        }
    }
}

impl Display for ParseLettersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseLetter(err) => write!(f, "failed to parse letter: {err}"),
            Self::WrongLength(err) => write!(f, "wrong length: {err}"),
        }
    }
}

impl From<ParseLetterError> for ParseLettersError {
    fn from(value: ParseLetterError) -> Self {
        Self::ParseLetter(value)
    }
}

impl From<crate::array::LengthError> for ParseLettersError {
    fn from(value: crate::array::LengthError) -> Self {
        Self::WrongLength(value)
    }
}
