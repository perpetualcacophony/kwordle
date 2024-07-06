use std::{ops::Deref, str::FromStr};

use crate::{Array, Letter};

mod error;
pub use error::ParseLettersError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde_derive",
    serde(bound = "Array<Letter, N>: serde::Serialize + for<'a> serde::Deserialize<'a>")
)]
pub struct Letters<const N: usize> {
    array: Array<Letter, N>,
}

impl<const N: usize> Letters<N> {
    pub const fn new(array: Array<Letter, N>) -> Self {
        Self { array }
    }
}

impl<const N: usize> Deref for Letters<N> {
    type Target = Array<Letter, N>;

    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl<const N: usize> IntoIterator for Letters<N> {
    type Item = Letter;
    type IntoIter = <Array<Letter, N> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}

impl<const N: usize> FromStr for Letters<N> {
    type Err = ParseLettersError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Array::try_from(
            s.chars()
                .map(Letter::try_from)
                .collect::<Result<Vec<Letter>, _>>()
                .map_err(ParseLettersError::ParseLetter)?,
        )
        .map_err(ParseLettersError::from)
        .map(Self::new)
    }
}
