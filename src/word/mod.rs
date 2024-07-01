use std::{fmt::Display, str::FromStr};

use crate::{
    letter::letters::{Letters, ParseLettersError},
    LetterState,
};

mod letters_map;
pub use letters_map::LettersMap;

mod error;
pub use error::ParseWordError;

pub mod list;

mod validity;
pub use validity::WordValidity;

mod words;

#[cfg(test)]
pub mod constants;

#[allow(unused_imports)]
pub use list::WordsList;

/// Represents a single valid word from a specific [`WordsList`].
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word<const LEN: usize> {
    letters: Letters<LEN>,
}

impl<const LEN: usize> Word<LEN> {
    pub(crate) fn new_unchecked(letters: Letters<LEN>) -> Self {
        Self { letters }
    }

    #[allow(dead_code)]
    pub(crate) fn from_str_unchecked(s: &str) -> Result<Self, ParseLettersError> {
        let letters = Letters::from_str(s)?;
        Ok(Self::new_unchecked(letters))
    }

    /// Constructs a new `Word` from a [`Letters`] object if it exists in the given [`WordsList`].
    ///
    /// # Errors
    /// Returns [`ParseWordError::NotInList`] if the word cannot be found in the list.
    pub fn from_letters(
        list: &WordsList<LEN>,
        letters: Letters<LEN>,
    ) -> Result<Self, error::ParseWordError> {
        let unchecked = Self::new_unchecked(letters);

        if list.guessable.contains(unchecked) {
            Ok(unchecked)
        } else {
            Err(error::ParseWordError::NotInList {
                letters: unchecked.letters.into(),
            })
        }
    }

    /// Constructs a new `Word` by parsing a string slice.
    ///
    /// # Errors
    /// Returns a [`ParseLettersError`] if parsing the string fails,
    /// or [`ParseWordError::NotInList`] if the word cannot be found in the list.
    pub fn from_str(list: &WordsList<LEN>, s: &str) -> Result<Self, error::ParseWordError> {
        let letters = Letters::from_str(s)?;
        Self::from_letters(list, letters)
    }

    /// Constructs a new [`LettersMap`] from this word's letters.
    pub fn letters_map(self) -> letters_map::LettersMap {
        letters_map::LettersMap::from_iter(self.letters)
    }

    /// Checks the letters of another `Word` against this one,
    /// returning a [`Guess`](super::Guess) with the status of each guessed letter.
    pub fn guess_word(self, word: Self) -> super::guess::Guess<LEN> {
        let mut guess = crate::guess::Guess::none_present(word.letters);
        let mut map = self.letters_map();

        for (guess, answer) in guess.iter_mut().zip(self.letters.into_iter()) {
            if guess.0 == answer {
                guess.1 = LetterState::Correct;
                map.decrement(guess.0);
            }
        }

        for guess in guess.iter_mut() {
            if map.contains_letter(guess.0) {
                guess.1 = LetterState::WrongPlace;
                map.decrement(guess.0);
            }
        }

        guess
    }

    /// Parses a string slice into a `Word` with the given [`WordsList`],
    /// and then guesses that `Word` against this one.
    ///
    /// # Errors
    /// Returns a [`ParseWordError`] if parsing the string into a `Word` fails.
    pub fn guess_str(
        self,
        list: &WordsList<LEN>,
        s: &str,
    ) -> Result<super::Guess<LEN>, error::ParseWordError> {
        let word = Self::from_str(list, s)?;
        Ok(self.guess_word(word))
    }
}

impl<const LEN: usize> Display for Word<LEN> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for letter in self.letters {
            Display::fmt(&letter, f)?;
        }

        Ok(())
    }
}

impl<const LEN: usize> PartialEq<str> for Word<LEN> {
    fn eq(&self, other: &str) -> bool {
        self.to_string().as_str().eq(other)
    }
}

impl<'s, const LEN: usize> PartialEq<&'s str> for Word<LEN> {
    fn eq(&self, other: &&'s str) -> bool {
        self.eq(*other)
    }
}

impl<'w, const LEN: usize> PartialEq<&'w Self> for Word<LEN> {
    fn eq(&self, other: &&'w Self) -> bool {
        self.eq(*other)
    }
}

impl<'w, const LEN: usize> PartialEq<Word<LEN>> for &'w Word<LEN> {
    fn eq(&self, other: &Word<LEN>) -> bool {
        (*self).eq(other)
    }
}

impl<const LEN: usize> PartialEq<Letters<LEN>> for Word<LEN> {
    fn eq(&self, other: &Letters<LEN>) -> bool {
        self.letters.eq(other)
    }
}

#[cfg(test)]
mod tests {
    use super::Word;

    fn amber() -> Word<5> {
        Word::from_str_unchecked("amber").expect("'amber' should be a valid 5-letter Word")
    }

    #[test]
    fn to_string() {
        assert_eq!(amber().to_string().as_str(), "amber");
    }

    #[test]
    fn partial_eq_str() {
        assert_eq!(amber(), "amber")
    }
}
