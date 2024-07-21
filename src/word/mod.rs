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
#[derive(Copy, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word<const LEN: usize = 5> {
    letters: Letters<LEN>,
}

impl<const LEN: usize> Word<LEN> {
    pub unsafe fn new_unchecked(letters: Letters<LEN>) -> Self {
        Self { letters }
    }

    #[allow(dead_code)]
    pub unsafe fn from_str_unchecked(s: &str) -> Result<Self, ParseLettersError> {
        let letters = Letters::from_str(s)?;
        Ok(Self::new_unchecked(letters))
    }

    #[cfg(feature = "serde")]
    pub fn serialize_as_str<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }

    /// Constructs a new `Word` from a [`Letters`] object if it exists in the given [`WordsList`].
    ///
    /// # Errors
    /// Returns [`ParseWordError::NotInList`] if the word cannot be found in the list.
    pub fn from_letters(
        list: &WordsList<LEN>,
        letters: Letters<LEN>,
    ) -> Result<Self, error::ParseWordError> {
        if list.guessable.contains_letters(letters) {
            unsafe { Ok(Self::new_unchecked(letters)) }
        } else {
            Err(error::ParseWordError::NotInList {
                letters: letters.to_vec(),
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
    pub fn guess(self, word: Self) -> super::guess::Guess<LEN> {
        let mut guess = crate::guess::Guess::none_present(word.letters);
        let mut map = self.letters_map();

        for ((letter, state), answer) in guess.iter_mut().zip(self.letters.into_iter()) {
            if letter == answer {
                *state = LetterState::Correct;
                map.decrement(letter);
            }
        }

        for (letter, state) in guess.iter_mut() {
            // i don't know why i need to check for correctness again? whatev
            if map.contains_letter(letter) && *state != LetterState::Correct {
                *state = LetterState::WrongPlace;
                map.decrement(letter);
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
        Ok(self.guess(word))
    }
}

impl<const N: usize> Display for Word<N>
where
    Letters<N>: Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for letter in self.letters {
            Display::fmt(&letter, f)?;
        }

        Ok(())
    }
}

impl<const LEN: usize> From<Word<LEN>> for Letters<LEN> {
    fn from(value: Word<LEN>) -> Self {
        value.letters
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
    use super::constants as words;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn to_string() {
        assert_eq!(words::AMBER().to_string().as_str(), "amber");
        assert_ne!(words::SONAR().to_string().as_str(), "amber");
    }

    #[test]
    fn partial_eq_str() {
        assert_eq!(words::AMBER(), "amber");
        assert_ne!(words::SONAR(), "amber")
    }

    mod letters_map {
        use crate::Letter;

        use super::{assert_eq, words};

        #[test]
        fn amber() {
            let word = words::AMBER();
            let map = word.letters_map();

            for letter in word.letters {
                assert!(map.contains_letter(letter));
                assert_eq!(map.count_letter(letter), 1);
            }
        }

        #[test]
        fn mummy() {
            let word = words::MUMMY();
            let map = word.letters_map();

            for letter in word.letters {
                assert!(map.contains_letter(letter))
            }

            assert_eq!(map.count_letter(Letter::M), 3);
            assert_eq!(map.count_letter(Letter::U), 1);
            assert_eq!(map.count_letter(Letter::Y), 1);
        }

        #[test]
        fn tummy() {
            let word = words::TUMMY();
            let map = word.letters_map();

            for letter in word.letters {
                assert!(map.contains_letter(letter))
            }

            assert_eq!(map.count_letter(Letter::T), 1);
            assert_eq!(map.count_letter(Letter::U), 1);
            assert_eq!(map.count_letter(Letter::M), 2);
            assert_eq!(map.count_letter(Letter::Y), 1);
        }
    }
}
