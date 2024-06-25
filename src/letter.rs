use std::{fmt::Write, str::FromStr};

macro_rules! match_letter {
    ($var:expr; $($letter:ident => $expr:expr),+) => {
        match $var {
            $(
                Letter::$letter => $expr
            ),+
        }
    };
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z
}

impl Letter {
    pub fn as_char(&self) -> char {
        (*self).to_char()
    }

    pub fn to_char(self) -> char {
        self.into()
    }
}

impl From<Letter> for char {
    fn from(value: Letter) -> Self {
        match_letter!(value; A => 'a', B => 'b', C => 'c', D => 'd', E => 'e', F => 'f', G => 'g', H => 'h', I => 'i', J => 'j', K => 'k', L => 'l', M => 'm', N => 'n', O => 'o', P => 'p', Q => 'q', R => 'r', S => 's', T => 't', U => 'u', V => 'v', W => 'w', X => 'x', Y => 'y', Z => 'z')
    }
}

impl std::fmt::Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

impl TryFrom<char> for Letter {
    type Error = ParseLetterError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let lowercase = value.to_ascii_lowercase();

        match lowercase {
            'a' => Ok(Letter::A),
            'b' => Ok(Letter::B),
            'c' => Ok(Letter::C),
            'd' => Ok(Letter::D),
            'e' => Ok(Letter::E),
            'f' => Ok(Letter::F),
            'g' => Ok(Letter::G),
            'h' => Ok(Letter::H),
            'i' => Ok(Letter::I),
            'j' => Ok(Letter::J),
            'k' => Ok(Letter::K),
            'l' => Ok(Letter::L),
            'm' => Ok(Letter::M),
            'n' => Ok(Letter::N),
            'o' => Ok(Letter::O),
            'p' => Ok(Letter::P),
            'q' => Ok(Letter::Q),
            'r' => Ok(Letter::R),
            's' => Ok(Letter::S),
            't' => Ok(Letter::T),
            'u' => Ok(Letter::U),
            'v' => Ok(Letter::V),
            'w' => Ok(Letter::W),
            'x' => Ok(Letter::X),
            'y' => Ok(Letter::Y),
            'z' => Ok(Letter::Z),
            other => Err(ParseLetterError::invalid_char(other))
        }
    }
}

impl FromStr for Letter {
    type Err = ParseLetterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ch = char::from_str(s).map_err(ParseLetterError::parse_char)?;
        Self::try_from(ch)
    }
}

pub enum ParseLetterError {
    InvalidChar(char),
    ParseChar(std::char::ParseCharError),
}

impl ParseLetterError {
    pub fn invalid_char(value: char) -> Self {
        Self::InvalidChar(value)
    }

    pub fn parse_char(error: std::char::ParseCharError) -> Self {
        Self::ParseChar(error)
    }
}