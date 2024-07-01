use std::{collections::HashSet, str::FromStr};

use crate::{word::words::Words, Word};

use super::guessable::Guessable;

type Base<const N: usize> = Words<N>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answers<const N: usize> {
    base: Base<N>,
}

impl<const N: usize> Answers<N> {
    unsafe fn new_unchecked(words: Words<N>) -> Self {
        Self { base: words }
    }

    fn try_new(words: Words<N>) -> Option<Self> {
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
    type IntoIter = <Base<N> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.base.into_iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a Answers<N> {
    type Item = &'a Word<N>;
    type IntoIter = <&'a Base<N> as IntoIterator>::IntoIter;

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
            .as_slice()
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
    type Err = <Base<N> as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(unsafe { Self::new_unchecked(Base::from_str(s)?) })
    }
}
