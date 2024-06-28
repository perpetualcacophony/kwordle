use std::ops::Deref;

use crate::{
    words_list::{GuessesAndAnswersList, HashSetWordsList},
    WordsList,
};

const ALLOWED: &str = include_str!("allowed.txt");
const ANSWERS: &str = include_str!("answers.txt");

pub struct Classic(GuessesAndAnswersList<Allowed, Answers, 5>);

impl Classic {
    fn new() -> Self {
        Self(GuessesAndAnswersList::new(Allowed::new(), Answers::new()))
    }
}

struct Allowed(HashSetWordsList<5>);

impl Allowed {
    fn new() -> Self {
        Self(HashSetWordsList::from_str(ALLOWED).expect("allowed.txt should be valid words"))
    }
}

impl From<HashSetWordsList<5>> for Allowed {
    fn from(value: HashSetWordsList<5>) -> Self {
        Self(value)
    }
}

impl Deref for Allowed {
    type Target = HashSetWordsList<5>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Answers(HashSetWordsList<5>);

impl Answers {
    fn new() -> Self {
        Self(HashSetWordsList::from_str(ANSWERS).expect("answers.txt should be valid words"))
    }
}

impl Deref for Answers {
    type Target = HashSetWordsList<5>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<HashSetWordsList<5>> for Answers {
    fn from(value: HashSetWordsList<5>) -> Self {
        Self(value)
    }
}
