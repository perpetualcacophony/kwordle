use crate::letter::Letter;

use crate::letters::ParseLettersError;

#[derive(Debug, Clone)]
pub enum ParseWordError {
    ParseLetters(ParseLettersError),
    NotInList { letters: Vec<Letter> },
}

impl From<ParseLettersError> for ParseWordError {
    fn from(value: ParseLettersError) -> Self {
        Self::ParseLetters(value)
    }
}

impl std::error::Error for ParseWordError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ParseLetters(err) => Some(err),
            _ => None,
        }
    }
}

impl std::fmt::Display for ParseWordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseLetters(err) => write!(f, "{err}"),
            Self::NotInList { letters } => write!(f, "letters {letters:?} not found in list"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ParseWordError;

    #[test]
    fn impl_error() {
        use assert_impl::assert_impl;
        use std::error::Error;

        assert_impl!(Error: ParseWordError)
    }
}
