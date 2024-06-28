mod collection;
mod core;
mod error;
pub mod structs;
use std::str::FromStr;

pub use error::ParseWordsListError;

use collection::WordsListCollection;

use crate::{Letters, Word};

#[allow(private_bounds)] // sealed trait
pub trait WordsList<const WORD_LEN: usize>
where
    Self: core::WordsListCore<WORD_LEN>,
{
    #[cfg(feature = "rand")]
    fn random_with<R>(&self, rng: &mut R) -> Word<WORD_LEN>
    where
        R: rand::Rng,
    {
        self.collection()
            .random(rng)
            .expect("type implementing WordsList must never be empty")
    }

    #[cfg(feature = "rand_full")]
    fn random(&self) -> Word<WORD_LEN> {
        self.random_with(&mut rand::thread_rng())
    }

    fn from_words<W>(words: W) -> Result<Self, ParseWordsListError>
    where
        Self: Sized,
        W: IntoIterator<Item = Word<WORD_LEN>>,
    {
        let collection = Self::Collection::from_words(words);

        if collection.is_empty() {
            return Err(ParseWordsListError::EmptyInput);
        }

        Ok(Self::from_collection(collection))
    }

    fn from_str(s: &str) -> Result<Self, ParseWordsListError>
    where
        Self: Sized,
    {
        if s.is_empty() {
            return Err(ParseWordsListError::EmptyInput);
        }

        let words = s
            .lines()
            .map(Letters::from_str)
            .collect::<Result<Vec<Letters<WORD_LEN>>, _>>()?
            .into_iter()
            .map(Word::new_unchecked);

        Self::from_words(words)
    }
}

impl<T, const WORD_LEN: usize> WordsList<WORD_LEN> for T where T: core::WordsListCore<WORD_LEN> {}
