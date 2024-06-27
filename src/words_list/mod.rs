use crate::{letters::ParseLettersError, word::Word};

mod core;
pub use core::WordsListCore;

mod hash_set;
pub use hash_set::HashSetWordsList;

/**
A trait for using a specific set of words.

To simplify the usage of this API, a collection implementing this trait
**must not** be empty. Methods in this trait may panic if used on
 an empty collection.

 Typically, this trait will not require any method definitions:
 ```
 # struct MyWordsList<const N: usize>;
 # use kwordle::{WordsList, Word};
# impl<const N: usize> kwordle::words_list::WordsListCore<N> for MyWordsList<N> {
#       fn is_empty(&self) -> bool {
#           false
#        }
#
#        fn try_random<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Option<Word<N>> {
#            None
#        }
#
#        fn collection_contains(&self, word: Word<N>) -> bool {
#            false
#        }
#
#        fn from_words<It: IntoIterator<Item = Word<N>>>(words: It) -> Self {
#            Self
#        }
 # }


 impl<const N: usize> WordsList<N> for MyWordsList<N> {}
 ```
 However, take care that a collection implementing this trait
 cannot be created with no items.

 ```should_panic
 # use kwordle::{Word, WordsList};

 # struct HashSet<T>(Option<T>);

 # impl<T> HashSet<T> {
 # fn new() -> Self {
 #     Self(None)
 #   }
 # }

 # impl<const N: usize> kwordle::words_list::WordsListCore<N> for HashSet<Word<N>> {
#       fn is_empty(&self) -> bool {
#           false
#        }
#
#        fn try_random<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Option<Word<N>> {
#            None
#        }
#
#        fn collection_contains(&self, word: Word<N>) -> bool {
#            false
#        }
#
#        fn from_words<It: IntoIterator<Item = Word<N>>>(words: It) -> Self {
#            Self(None)
#        }
 # }

 impl<const N: usize> WordsList<N> for HashSet<Word<N>> {}

 let list = HashSet::<Word<5>>::new();
 let word = list.random(); // This panics!
 ```

 The safest option is to implement this trait for a wrapper struct.
 ```no_run
# use kwordle::{Word, WordsList};
# use std::collections::HashSet;

 struct MyWordsList<const N: usize> {
     hash_set: HashSet<Word<N>>
 }

# impl kwordle::words_list::WordsListCore<5> for MyWordsList<5> {
#       fn is_empty(&self) -> bool {
#           false
#        }
#
#        fn try_random<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Option<Word<5>> {
#            None
#        }
#
#        fn collection_contains(&self, word: Word<5>) -> bool {
#            false
#        }
#
#        fn from_words<It: IntoIterator<Item = Word<5>>>(words: It) -> Self {
#            Self { hash_set: std::default::Default::default() }
#        }
 # }

 impl WordsList<5> for MyWordsList<5> {}

 let list = MyWordsList::<5>::from_str("/* words here... */")?;
 let word = list.random(); // This works!

 # Ok::<_, kwordle::words_list::ParseWordsListError>(())
 ```
*/
pub trait WordsList<const WORD_LEN: usize>: WordsListCore<WORD_LEN> {
    /// Checks if this list contains the given word.
    fn contains(&self, word: Word<WORD_LEN>) -> bool {
        WordsListCore::collection_contains(self, word)
    }

    /// Creates a words list from a string slice
    /// of words separated by newlines.
    ///
    /// # Errors
    /// Returns [`ParseWordsListError::EmptyInput`] if
    /// the string is empty, or
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
    /// Consider using [`random`](WordsList::random) if you don't need
    /// to use a cached [`Rng`](rand::Rng).
    ///
    /// # Panics
    /// Panics if the list is empty.
    fn random_with<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Word<WORD_LEN> {
        WordsListCore::try_random(self, rng).expect("should not be empty")
    }

    /// Returns a random [`Word`] from this list,
    /// using [`rand::thread_rng`].
    ///
    /// If many random words are needed, it might be more efficient
    /// to use [`random_with`](WordsList::random_with) and provide a single [`Rng`](rand::Rng).
    ///
    /// # Panics
    /// Panics if the list is empty.
    fn random(&self) -> Word<WORD_LEN> {
        self.random_with(&mut rand::thread_rng())
    }
}

#[derive(Debug, Clone)]
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
