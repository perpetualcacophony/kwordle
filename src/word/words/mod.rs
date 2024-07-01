use std::{ops::Deref, str::FromStr};

use crate::{letter::letters::ParseLettersError, Letters, Word};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Words<const N: usize> {
    base: Box<[Word<N>]>,
}

/// Constructor methods.
impl<const N: usize> Words<N> {
    pub fn from_vec(vec: Vec<Word<N>>) -> Self {
        Self::from_boxed_slice(vec.into_boxed_slice())
    }

    pub fn from_boxed_slice(boxed: Box<[Word<N>]>) -> Self {
        Self { base: boxed }
    }
}

impl<const N: usize> Deref for Words<N> {
    type Target = [Word<N>];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<const N: usize> AsRef<[Word<N>]> for Words<N> {
    fn as_ref(&self) -> &[Word<N>] {
        self.as_slice()
    }
}

impl<const N: usize> From<Vec<Word<N>>> for Words<N> {
    fn from(value: Vec<Word<N>>) -> Self {
        Self::from_vec(value)
    }
}

impl<const N: usize> From<Box<[Word<N>]>> for Words<N> {
    fn from(value: Box<[Word<N>]>) -> Self {
        Self::from_boxed_slice(value)
    }
}

impl<const N: usize> FromIterator<Word<N>> for Words<N> {
    fn from_iter<T: IntoIterator<Item = Word<N>>>(iter: T) -> Self {
        Self::from_boxed_slice(iter.into_iter().collect())
    }
}

impl<const N: usize> FromStr for Words<N> {
    type Err = ParseLettersError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.lines()
            .map(Letters::from_str)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(Word::new_unchecked)
            .collect())
    }
}

impl<const N: usize> Words<N> {
    pub fn into_boxed_slice(self) -> Box<[Word<N>]> {
        self.base
    }

    pub fn into_vec(self) -> Vec<Word<N>> {
        self.into_boxed_slice().into_vec()
    }

    pub fn as_slice(&self) -> &[Word<N>] {
        self.base.as_ref()
    }

    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl<const N: usize> IntoIterator for Words<N> {
    type Item = Word<N>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a Words<N> {
    type Item = &'a Word<N>;
    type IntoIter = std::slice::Iter<'a, Word<N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().into_iter()
    }
}
