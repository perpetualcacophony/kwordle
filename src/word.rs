use std::{collections::HashMap, fmt::Display, str::FromStr};

use crate::{
    letter::Letter,
    letter_state::LetterState,
    letters::{Letters, ParseLettersError},
};

#[allow(unused_imports)]
use crate::WordsList;

/// Represents a single valid word from a specific [`WordsList`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Word<const LEN: usize> {
    letters: Letters<LEN>,
}

impl<const LEN: usize> Word<LEN> {
    pub(crate) fn new_unchecked(letters: Letters<LEN>) -> Self {
        Self { letters }
    }

    /// Constructs a new `Word` from a [`Letters`] object if it exists in the given [`WordsList`].
    ///
    /// # Errors
    /// Returns [`ParseWordError::NotInList`] if the word cannot be found in the list.
    pub fn from_letters(
        list: &impl crate::WordsList<LEN>,
        letters: Letters<LEN>,
    ) -> Result<Self, ParseWordError> {
        let unchecked = Self::new_unchecked(letters);

        if list.contains(unchecked) {
            Ok(unchecked)
        } else {
            Err(ParseWordError::NotInList {
                letters: unchecked.letters.into(),
            })
        }
    }

    /// Constructs a new `Word` by parsing a string slice.
    ///
    /// # Errors
    /// Returns a [`ParseLettersError`] if parsing the string fails,
    /// or [`ParseWordError::NotInList`] if the word cannot be found in the list.
    pub fn from_str(list: &impl crate::WordsList<LEN>, s: &str) -> Result<Self, ParseWordError> {
        let letters = Letters::from_str(s)?;
        Self::from_letters(list, letters)
    }

    /// Constructs a new [`LettersMap`] from this word's letters.
    pub fn letters_map(self) -> LettersMap {
        LettersMap::from_iter(self.letters)
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
        list: &impl crate::WordsList<LEN>,
        s: &str,
    ) -> Result<super::Guess<LEN>, ParseWordError> {
        let word = Self::from_str(list, s)?;
        Ok(self.guess_word(word))
    }
}

pub enum ParseWordError {
    ParseLetters(ParseLettersError),
    NotInList { letters: Vec<Letter> },
}

impl From<ParseLettersError> for ParseWordError {
    fn from(value: ParseLettersError) -> Self {
        Self::ParseLetters(value)
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

#[derive(Default, Debug, Clone)]
pub struct LettersMap {
    hash_map: HashMap<Letter, usize>,
}

impl LettersMap {
    pub fn new() -> Self {
        HashMap::new().into()
    }

    pub fn count_letter(&self, letter: Letter) -> usize {
        self.hash_map.get(&letter).copied().unwrap_or_default()
    }

    pub fn contains_letter(&self, letter: Letter) -> bool {
        self.hash_map.contains_key(&letter)
    }

    pub fn insert(&mut self, letter: Letter) {
        self.hash_map.insert(letter, 1);
    }

    pub fn increment(&mut self, letter: Letter) {
        if let Some(count) = self.hash_map.get_mut(&letter) {
            *count += 1;
        } else {
            self.insert(letter)
        }
    }

    pub fn decrement(&mut self, letter: Letter) -> Option<usize> {
        if self.contains_letter(letter) {
            if self.count_letter(letter) == 1 {
                self.hash_map.remove(&letter);
                Some(0)
            } else {
                let count = self
                    .hash_map
                    .get_mut(&letter)
                    .expect("already checked that the map contains this letter");
                *count -= 1; // will never be 0
                Some(*count)
            }
        } else {
            None
        }
    }
}

impl From<HashMap<Letter, usize>> for LettersMap {
    fn from(value: HashMap<Letter, usize>) -> Self {
        Self { hash_map: value }
    }
}

impl FromIterator<Letter> for LettersMap {
    fn from_iter<T: IntoIterator<Item = Letter>>(iter: T) -> Self {
        let mut map = Self::new();

        for letter in iter {
            map.increment(letter);
        }

        map
    }
}
