use std::{collections::BTreeSet, fmt::Write, str::FromStr};

pub mod letters;
pub use letters::Letters;

mod error;
pub use error::ParseLetterError;

pub const ALPHABET: std::ops::RangeInclusive<Letter> = Letter::A..=Letter::Z;

impl std::iter::Step for Letter {
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        std::iter::Step::forward_checked(char::from(start), count).and_then(Self::from_char)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        std::iter::Step::backward_checked(char::from(start), count).and_then(Self::from_char)
    }

    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        std::iter::Step::steps_between(&char::from(*start), &char::from(*end))
    }
}

pub fn alphabet_set() -> BTreeSet<Letter> {
    let mut set = BTreeSet::new();

    for letter in ALPHABET {
        set.insert(letter);
    }

    set
}

#[cfg(feature = "serde")]
mod serde;

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
    Z,
}

impl Letter {
    pub fn as_char(self) -> char {
        self.into()
    }

    pub const fn from_char(ch: char) -> Option<Self> {
        let lowercase = ch.to_ascii_lowercase();

        match lowercase {
            'a' => Some(Letter::A),
            'b' => Some(Letter::B),
            'c' => Some(Letter::C),
            'd' => Some(Letter::D),
            'e' => Some(Letter::E),
            'f' => Some(Letter::F),
            'g' => Some(Letter::G),
            'h' => Some(Letter::H),
            'i' => Some(Letter::I),
            'j' => Some(Letter::J),
            'k' => Some(Letter::K),
            'l' => Some(Letter::L),
            'm' => Some(Letter::M),
            'n' => Some(Letter::N),
            'o' => Some(Letter::O),
            'p' => Some(Letter::P),
            'q' => Some(Letter::Q),
            'r' => Some(Letter::R),
            's' => Some(Letter::S),
            't' => Some(Letter::T),
            'u' => Some(Letter::U),
            'v' => Some(Letter::V),
            'w' => Some(Letter::W),
            'x' => Some(Letter::X),
            'y' => Some(Letter::Y),
            'z' => Some(Letter::Z),
            _ => None,
        }
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
            other => Err(ParseLetterError::invalid_char(other)),
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
