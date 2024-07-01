use std::{char::ParseCharError, error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum ParseLetterError {
    InvalidChar(char),
    ParseChar(std::char::ParseCharError),
}

impl ParseLetterError {
    pub const fn invalid_char(value: char) -> Self {
        Self::InvalidChar(value)
    }

    pub const fn parse_char(error: std::char::ParseCharError) -> Self {
        Self::ParseChar(error)
    }
}

impl Error for ParseLetterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParseChar(err) => Some(err),
            _ => None,
        }
    }
}

impl Display for ParseLetterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChar(ch) => write!(f, "could not parse char '{ch}' as a letter"),
            Self::ParseChar(err) => write!(f, "failed to parse char: {err}"),
        }
    }
}

impl From<ParseCharError> for ParseLetterError {
    fn from(value: ParseCharError) -> Self {
        Self::ParseChar(value)
    }
}
