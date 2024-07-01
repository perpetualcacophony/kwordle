mod collection;
//pub mod core;
//pub use core::WordsListCore;

mod error;

pub mod structs;
pub use structs::HashSetWordsList;

pub use error::ParseWordsListError;

pub use collection::Collection;

//pub use crate::Word;

type Word<const LEN: usize, List: WordsList<LEN>> =
    <List::Collection as collection::Adapter<LEN>>::Word;

pub trait WordsList<const WORD_LEN: usize> {
    type Collection: collection::Adapter<WORD_LEN>;

    fn collection(&self) -> &Collection<WORD_LEN, Self::Collection>;

    fn collection_mut(&mut self) -> &mut Collection<WORD_LEN, Self::Collection>;

    fn from_collection(collection: Collection<WORD_LEN, Self::Collection>) -> Self;

    #[cfg(feature = "rand")]
    fn random_with<R>(&self, rng: &mut R) -> Word<WORD_LEN, Self>
    where
        R: rand::Rng,
    {
        self.collection().random_with(rng)
    }

    #[cfg(feature = "rand_full")]
    fn random(&self) -> Word<WORD_LEN, Self> {
        self.random_with(&mut rand::thread_rng())
    }

    fn from_words<W>(words: W) -> Option<Self>
    where
        Self: Sized,
        W: IntoIterator<Item = Word<WORD_LEN, Self>>,
    {
        Some(Self::from_collection(Collection::from_words(words)?))
    }

    fn from_str(s: &str) -> Result<Self, ParseWordsListError>
    where
        Self: Sized,
        Self::Collection: collection::Adapter<WORD_LEN, Word = crate::Word<WORD_LEN>>,
    {
        let collection = Collection::from_str(s)?;
        Ok(Self::from_collection(collection))
    }

    fn contains(&self, word: crate::Word<WORD_LEN>) -> bool {
        self.collection().contains(word)
    }

    fn answers(&self) -> collection::BorrowingCollection<'_, WORD_LEN, Self::Collection>
    where
        Self::Collection: collection::adapter::Owning<WORD_LEN>,
    {
        self.collection().borrowing()
    }

    fn push(&mut self, word: Word<WORD_LEN, Self>) -> &Word<WORD_LEN, Self> {
        self.collection_mut().push(word)
    }
}

pub mod guessable;
