use std::{collections::HashSet, ops::Deref};

use crate::{word::list::{Collection, HashSetWordsList}, Word, WordsList as _};

const ALLOWED: &str = include_str!("allowed.txt");
const ANSWERS: &str = include_str!("answers.txt");

pub struct WordsList<'w> {
    allowed: Collection<5, HashSet<Word<5>>>,
    answers: Collection<5, HashSet<&'w Word<5>>>,
}

impl<'w> WordsList<'w> {
    pub fn new() -> Self {
        let mut allowed = Collection::from_str(ALLOWED).unwrap();
        let answers_owned = HashSetWordsList::from_str(ANSWERS).unwrap();
        let mut answers_set = HashSet::new();

        for word in answers_owned {
            let answer_ref = allowed.push(word.clone());
            answers_set.insert(answer_ref);
        }

        let answers = Collection::try_new(answers_set).unwrap();

        Self { allowed, answers }
    }
}

impl crate::WordsList<5> for WordsList<'_> {
    type Collection = HashSet<Word<5>>;

    fn collection(&self) -> &crate::word::list::Collection<5, Self::Collection> {
        &self.allowed
    }

    fn collection_mut(&mut self) -> &mut Collection<5, Self::Collection> {
        &mut self.allowed
    }

    fn from_collection(collection: crate::word::list::Collection<5, Self::Collection>) -> Self {
        
    }

    #[cfg(feature = "rand")]
    fn random_answer_with<R>(&self, rng: &mut R) -> Option<Word<5>>
        where
            R: rand::Rng, {
        self.
    }
}