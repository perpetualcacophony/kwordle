use std::{collections::HashSet, str::FromStr};

use rand::seq::IteratorRandom;

use crate::{letters::ParseLettersError, Letters, Word};

/// This trait represents the collection-specific
/// behavior of a type implementing [WordsList].
/// 
/// Unlike WordsList, this trait **does not** require a non-empty collection.
pub trait WordsListCore<const WORD_LEN: usize> {
    /// Checks if this collection contains any items.
    fn is_empty(&self) -> bool;

    /// Returns a random item from the collection,
    /// or None if the collection is empty.
    /// 
    /// If this type is [`IntoIterator<Item = &Word>`],
    /// consider implementing this method using [`choose_random`].
    fn try_random<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Option<Word<WORD_LEN>>;
    
    /// Uses [`rand::seq::IteratorRandom`] to choose
    /// a random item from the collection,
    /// or None if the collection is empty.
    fn choose_random<R>(&self, rng: &mut R) -> Option<Word<WORD_LEN>>
    where
        R: rand::Rng + ?Sized,
        for<'a> &'a Self: IntoIterator<Item = &'a Word<WORD_LEN>>
    {
        self.into_iter().choose(rng).copied()
    }

    /// Creates a new instance of this collection
    /// from an iterator of words.
    fn from_words<It: IntoIterator<Item = Word<WORD_LEN>>>(words: It) -> Self;

    fn from_str_unchecked(s: &str) -> Result<Self, ParseLettersError> where Self: Sized {
        let words = s
            .lines()
            .map(Letters::from_str)
            .collect::<Result<Vec<Letters<WORD_LEN>>, _>>()?
            .into_iter()
            .map(Word::new_unchecked);

        Ok(Self::from_words(words))
    }

    /// Checks if this collection contains a given [`Word`].
    fn collection_contains(&self, word: Word<WORD_LEN>) -> bool;
}

impl<const N: usize> WordsListCore<N> for HashSet<Word<N>> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn try_random<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Option<Word<N>> {
        self.choose_random(rng)
    }

    fn collection_contains(&self, word: Word<N>) -> bool {
        self.contains(&word)
    }

    fn from_words<It: IntoIterator<Item = Word<N>>>(words: It) -> Self {
        words.into_iter().collect()
    }
}