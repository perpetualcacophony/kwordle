use std::{collections::HashSet, str::FromStr};

use crate::{letter::letters::ParseLettersError, Word};

use super::guessable::Guessable;

type Base<const N: usize> = Box<[Word<N>]>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answers<const N: usize> {
    base: Base<N>,
}

impl<const N: usize> Answers<N> {
    unsafe fn new_unchecked(words: Base<N>) -> Self {
        Self { base: words }
    }

    fn try_new(words: Base<N>) -> Option<Self> {
        if words.is_empty() {
            None
        } else {
            unsafe { Some(Self::new_unchecked(words)) }
        }
    }

    pub fn try_from_iter<I>(iter: I) -> Option<Self>
    where
        I: IntoIterator<Item = Word<N>>,
    {
        let base = iter.into_iter().collect();
        Self::try_new(base)
    }

    unsafe fn from_iter_unchecked<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Word<N>>,
    {
        let base = iter.into_iter().collect();
        Self::new_unchecked(base)
    }

    pub fn from_guessable(guessable: &Guessable<N>) -> Self {
        unsafe { Self::from_iter_unchecked(guessable.into_iter().copied()) }
    }
}

impl<const N: usize> IntoIterator for Answers<N> {
    type Item = Word<N>;
    type IntoIter = std::vec::IntoIter<Word<N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.base.into_vec().into_iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a Answers<N> {
    type Item = &'a Word<N>;
    type IntoIter = std::slice::Iter<'a, Word<N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.base.iter()
    }
}

impl<const N: usize> Answers<N> {
    pub fn append_to_guessable(&self, guessable: &mut super::guessable::Guessable<N>) {
        guessable.extend(self.clone())
    }

    pub fn as_slice(&self) -> &[Word<N>] {
        &self.base
    }

    pub fn into_set(self) -> HashSet<Word<N>> {
        self.into_iter().collect()
    }

    pub fn to_set(&self) -> HashSet<&Word<N>> {
        self.into_iter().collect()
    }

    #[cfg(feature = "rand")]
    pub fn random_with<R>(&self, rng: &mut R) -> Word<N>
    where
        R: rand::Rng,
    {
        use rand::seq::SliceRandom;

        self.base
            .choose(rng)
            .copied()
            .expect("Answers should not be empty")
    }

    #[cfg(feature = "rand_full")]
    pub fn random(&self) -> Word<N> {
        self.random_with(&mut rand::thread_rng())
    }
}

impl<const N: usize> FromStr for Answers<N> {
    type Err = ParseLettersError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(unsafe { Self::new_unchecked(crate::word::words::parse_from_str(s)?) })
    }
}
