use std::{collections::HashSet, ops::Deref};

use crate::{Word, WordsList as _};

const ALLOWED: &str = include_str!("allowed.txt");
const ANSWERS: &str = include_str!("answers.txt");

pub struct WordsList {
    allowed: Allowed,
    answers: Answers,
}

impl WordsList {
    pub fn new() -> Self {
        Self {
            allowed: Allowed::new(),
            answers: Answers::new(),
        }
    }
}

impl crate::WordsList<5> for WordsList {
    #[cfg(feature = "rand")]
    fn random_answer_with<R>(&self, rng: &mut R) -> Option<Word<5>>
        where
            R: rand::Rng, {
        self.
    }
}

struct Allowed {
    core: crate::word::list::core::HashSetCore<5>,
}

impl Allowed {
    fn new() -> Self {
        Self::from_str(ALLOWED).expect("allowed.txt should be valid 5-letter words")
    }
}

impl crate::WordsList<5> for Allowed {
    type Collection = HashSet<Word<5>>;

    fn core(&self) -> &crate::word::list::WordsListCore<Self::Collection, 5> {
        &self.core
    }

    fn from_core(core: crate::word::list::core::WordsListCore<Self::Collection, 5>) -> Self {
        Self { core }
    }
}

struct Answers {
    core: crate::word::list::core::HashSetCore<5>,
}

impl Answers {
    fn new() -> Self {
        Self::from_str(ANSWERS).expect("answers.txt should be valid 5-letter words")
    }
}

impl crate::WordsList<5> for Answers {
    type Collection = HashSet<Word<5>>;

    fn core(&self) -> &crate::word::list::WordsListCore<Self::Collection, 5> {
        &self.core
    }

    fn from_core(core: crate::word::list::core::WordsListCore<Self::Collection, 5>) -> Self {
        Self { core }
    }
}
