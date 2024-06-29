mod collection;
pub mod core;
pub use core::WordsListCore;

mod error;

pub mod structs;
pub use structs::HashSetWordsList;

pub use error::ParseWordsListError;

use collection::WordsListCollection;

pub use crate::Word;

pub trait WordsList<const WORD_LEN: usize> {
    type Collection: WordsListCollection<WORD_LEN>;

    fn core(&self) -> &WordsListCore<Self::Collection, WORD_LEN>;

    fn from_core(core: WordsListCore<Self::Collection, WORD_LEN>) -> Self;

    #[cfg(feature = "rand")]
    fn random_with<R>(&self, rng: &mut R) -> Word<WORD_LEN>
    where
        R: rand::Rng,
    {
        self.core().random_with(rng)
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
        Ok(Self::from_core(
            <WordsListCore<Self::Collection, WORD_LEN>>::from_words(words)?,
        ))
    }

    fn from_str(s: &str) -> Result<Self, ParseWordsListError>
    where
        Self: Sized,
    {
        Ok(Self::from_core(
            <WordsListCore<Self::Collection, WORD_LEN>>::from_str(s)?,
        ))
    }

    fn contains(&self, word: Word<WORD_LEN>) -> bool {
        self.core().contains(word)
    }

    #[cfg(feature = "rand")]
    fn random_answer_with<R>(&self, rng: &mut R) -> Option<Word<WORD_LEN>>
    where
        R: rand::Rng,
    {
        Some(self.random_with(rng))
    }

    #[cfg(feature = "rand_full")]
    fn random_answer(&self) -> Option<Word<WORD_LEN>> {
        self.random_answer_with(&mut rand::thread_rng())
    }
}
