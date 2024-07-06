use std::{collections::HashSet, str::FromStr};

use crate::{Letters, Word};

use super::{answers::Answers, ParseWordsListError};

#[derive(Debug, Clone, PartialEq)]
pub struct Guessable<const N: usize> {
    set: HashSet<Word<N>>,
}

/// Constructors
impl<const N: usize> Guessable<N> {
    const unsafe fn new_unchecked(set: HashSet<Word<N>>) -> Self {
        Self { set }
    }

    fn try_new(set: HashSet<Word<N>>) -> Option<Self> {
        if set.is_empty() {
            None
        } else {
            unsafe { Some(Self::new_unchecked(set)) }
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
        let words = crate::word::words::parse_from_str(s)?;
        Self::from_words(words.into_vec()).ok_or(ParseWordsListError::EmptyInput)
    }
}

impl<const N: usize> Guessable<N> {
    pub fn contains(&self, word: Word<N>) -> bool {
        self.set.contains(&word)
    }

    pub fn contains_letters(&self, letters: Letters<N>) -> bool {
        self.contains(Word::new_unchecked(letters))
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, Word<N>> {
        self.into_iter()
    }

    pub fn includes_answers(&self, answers: &Answers<N>) -> bool {
        self.answers_intersection(answers) == answers.to_set()
    }

    pub fn excludes_answers(&self, answers: &Answers<N>) -> bool {
        self.answers_intersection(answers).is_empty()
    }

    pub fn answers_intersection<'word>(
        &self,
        answers: &'word Answers<N>,
    ) -> HashSet<&'word Word<N>> {
        let mut intersection = HashSet::new();

        for word in answers {
            if self.contains(*word) {
                intersection.insert(word);
            }
        }

        intersection
    }
}

impl<const N: usize> Extend<Word<N>> for Guessable<N> {
    fn extend<T: IntoIterator<Item = Word<N>>>(&mut self, iter: T) {
        self.set.extend(iter)
    }
}

impl<'a, const N: usize> IntoIterator for &'a Guessable<N> {
    type Item = &'a Word<N>;
    type IntoIter = std::collections::hash_set::Iter<'a, Word<N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.iter()
    }
}
