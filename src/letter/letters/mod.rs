use std::{
    array::IntoIter,
    ops::{Index, IndexMut},
    str::FromStr,
};

use crate::Letter;

mod error;
pub use error::ParseLettersError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde_derive",
    serde(bound = "[L; N]: serde::Serialize + for<'a> serde::Deserialize<'a>")
)]
pub struct Letters<const N: usize, L = Letter> {
    array: [L; N],
}

impl<const N: usize> Letters<N> {
    pub const fn new(array: [Letter; N]) -> Self {
        Self { array }
    }
}

impl<const N: usize, L> Letters<N, L> {
    pub fn map<U>(self, f: impl FnMut(L) -> U) -> Letters<N, U> {
        let array = self.array.map(f);
        Letters { array }
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, L> {
        self.array.iter_mut()
    }
}

impl<const N: usize> From<Letters<N>> for [Letter; N] {
    fn from(value: Letters<N>) -> Self {
        value.array
    }
}

impl<const N: usize> From<Letters<N>> for Vec<Letter> {
    fn from(value: Letters<N>) -> Self {
        value.array.to_vec()
    }
}

pub type Standard = Letters<5>;

impl<const N: usize> From<[Letter; N]> for Letters<N> {
    fn from(value: [Letter; N]) -> Self {
        Self { array: value }
    }
}

impl<const N: usize> TryFrom<Vec<Letter>> for Letters<N> {
    type Error = ParseLettersError;

    fn try_from(value: Vec<Letter>) -> Result<Self, Self::Error> {
        let array: [Letter; N] =
            value
                .try_into()
                .map_err(|vec: Vec<Letter>| ParseLettersError::WrongLength {
                    expected: N,
                    got: vec.len(),
                })?;

        Ok(array.into())
    }
}

impl<const N: usize, L> Index<usize> for Letters<N, L> {
    type Output = L;

    fn index(&self, index: usize) -> &Self::Output {
        self.array.index(index)
    }
}

impl<const N: usize, L> IndexMut<usize> for Letters<N, L> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.array.index_mut(index)
    }
}

impl<const N: usize, L> IntoIterator for Letters<N, L> {
    type Item = L;
    type IntoIter = std::array::IntoIter<Self::Item, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}

impl<const N: usize> FromStr for Letters<N> {
    type Err = ParseLettersError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letters: Vec<Letter> = s
            .chars()
            .map(Letter::try_from)
            .collect::<Result<_, _>>()
            .map_err(ParseLettersError::ParseLetter)?;

        letters.try_into()
    }
}

impl<const N: usize, L: std::fmt::Debug> FromIterator<L> for Letters<N, L> {
    fn from_iter<T: IntoIterator<Item = L>>(iter: T) -> Self {
        let vec: Vec<L> = Vec::from_iter(iter);
        let array: [L; N] = <[L; N]>::try_from(vec).unwrap();
        Self { array }
    }
}
