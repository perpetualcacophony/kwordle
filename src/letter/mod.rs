use std::{collections::BTreeSet, fmt::Write};

pub mod letters;
pub use letters::Letters;

mod error;
pub use error::ParseLetterError;

pub const ALPHABET: std::ops::RangeInclusive<Letter> = Letter::A..=Letter::Z;

#[cfg(feature = "step")]
pub fn alphabet_set() -> BTreeSet<Letter> {
    let mut set = BTreeSet::new();

    for letter in ALPHABET {
        set.insert(letter);
    }

    set
}

#[cfg(feature = "serde")]
mod serde;

macro_rules! enum_letter {
    { $($name:ident $ch:literal),+ } => {
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
        #[cfg_attr(feature = "step", derive(derive_step::Step))]
        pub enum Letter {
            $($name),+
        }

        impl Letter {
            pub fn to_char(&self) -> char {
                match self {
                    $( Self::$name => $ch ),+
                }
            }

            pub fn from_char_lowercase(ch: char) -> Option<Self> {
                match ch {
                    $( $ch => Some(Self::$name) ),+,
                    _ => None,
                }
            }
        }
    };
}

enum_letter! {
    A 'a',
    B 'b',
    C 'c',
    D 'd',
    E 'e',
    F 'f',
    G 'g',
    H 'h',
    I 'i',
    J 'j',
    K 'k',
    L 'l',
    M 'm',
    N 'n',
    O 'o',
    P 'p',
    Q 'q',
    R 'r',
    S 's',
    T 't',
    U 'u',
    V 'v',
    W 'w',
    X 'x',
    Y 'y',
    Z 'z'
}

impl Letter {
    pub fn from_char(ch: char) -> Option<Self> {
        Self::from_char_lowercase(ch.to_ascii_lowercase())
    }
}

impl From<Letter> for char {
    fn from(value: Letter) -> Self {
        value.to_char()
    }
}

impl<'a> From<&'a Letter> for char {
    fn from(value: &'a Letter) -> Self {
        value.to_char()
    }
}

impl std::fmt::Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.into())
    }
}

impl TryFrom<char> for Letter {
    type Error = ParseLetterError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Self::from_char(value).ok_or(ParseLetterError::InvalidChar(value))
    }
}
