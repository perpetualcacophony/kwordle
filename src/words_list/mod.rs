use crate::{letters::ParseLettersError, word::Word};

mod core;
pub use core::WordsListCore;

mod hash_set;
pub use hash_set::HashSetWordsList;

/// A trait for using a specific set of words.
///
/// To simplify the usage of this API, a collection implementing this trait
/// **must not** be empty.
pub trait WordsList<const WORD_LEN: usize>: WordsListCore<WORD_LEN> {
    /// Checks if this list contains the given word.
    fn contains(&self, word: Word<WORD_LEN>) -> bool {
        WordsListCore::collection_contains(self, word)
    }

    /// Creates a words list from a string slice
    /// of words separated by newlines.
    ///
    /// ### Errors
    /// Will return [`ParseWordsListError::EmptyInput`] if
    /// the string is empty or no words are found, or
    /// [`ParseWordsListError::ParseLetters`] if the string
    /// cannot be parsed into words.
    fn from_str(s: &str) -> Result<Self, ParseWordsListError>
    where
        Self: Sized,
    {
        if s.is_empty() {
            return Err(ParseWordsListError::EmptyInput);
        }

        let unchecked = Self::from_str_unchecked(s)?;

        if unchecked.is_empty() {
            return Err(ParseWordsListError::EmptyInput);
        }

        Ok(unchecked)
    }

    /// Returns a random [`Word`] from this list.
    ///
    /// ### Panics
    /// Panics if the list is empty.
    fn random_with<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Word<WORD_LEN> {
        WordsListCore::try_random(self, rng).expect("should not be empty")
    }

    /// Returns a random [`Word`] from this list,
    /// using [`rand::thread_rng`].
    ///
    /// If many random words are needed, it might be more efficient
    /// to use [`random_with`] and provide a single [`Rng`](rand::Rng).
    ///
    /// ### Panics
    /// Panics if the list is empty.
    fn random(&self) -> Word<WORD_LEN> {
        self.random_with(&mut rand::thread_rng())
    }
}

pub enum ParseWordsListError {
    ParseLetters(ParseLettersError),
    EmptyInput,
}

impl From<ParseLettersError> for ParseWordsListError {
    fn from(value: ParseLettersError) -> Self {
        Self::ParseLetters(value)
    }
}

pub struct GuessesAndAnswersList<G, A, const N: usize> {
    guesses: G,
    answers: A,
}

impl<const N: usize, G: WordsList<N>, A: WordsList<N>> GuessesAndAnswersList<G, A, N> {
    fn new(guesses: G, answers: A) -> Self {
        Self { guesses, answers }
    }

    fn valid_guess(&self, word: Word<N>) -> bool {
        self.guesses.contains(word) || self.answers.contains(word)
    }

    fn random_answer(&self) -> Word<N> {
        self.answers.random()
    }
}

impl<const N: usize, L: WordsList<N> + Clone> GuessesAndAnswersList<L, L, N> {
    fn from_single(list: L) -> Self {
        Self::new(list.clone(), list)
    }
}
