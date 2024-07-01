use std::{collections::HashSet, str::FromStr};

use crate::{word::words::Words, Letters, Word};

use super::ParseWordsListError;

pub struct Guessable<const N: usize> {
    set: HashSet<Word<N>>,
}

/// Constructors
impl<const N: usize> Guessable<N> {
    fn new_unchecked(set: HashSet<Word<N>>) -> Self {
        Self { set }
    }

    fn try_new(set: HashSet<Word<N>>) -> Option<Self> {
        if set.is_empty() {
            None
        } else {
            Some(Self::new_unchecked(set))
        }
    }

    pub fn from_words<I>(words: I) -> Option<Self>
    where
        I: IntoIterator<Item = Word<N>>,
    {
        let set = words.into_iter().collect();
        Self::try_new(set)
    }
}

impl<const N: usize> FromStr for Guessable<N> {
    type Err = ParseWordsListError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = Words::from_str(s)?;
        Self::from_words(words).ok_or(ParseWordsListError::EmptyInput)
    }
}

impl<const N: usize> Guessable<N> {
    pub fn contains(&self, word: Word<N>) -> bool {
        self.set.contains(&word)
    }

    pub fn contains_letters(&self, letters: Letters<N>) -> bool {
        self.contains(Word::new_unchecked(letters))
    }
}

impl<const N: usize> Extend<Word<N>> for Guessable<N> {
    fn extend<T: IntoIterator<Item = Word<N>>>(&mut self, iter: T) {
        self.set.extend(iter)
    }
}
