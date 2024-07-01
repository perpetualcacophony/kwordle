use std::str::FromStr;

use crate::word::list::{answers::Answers, guessable::Guessable, WordsList};

const ALLOWED: &str = include_str!("allowed.txt");
const ANSWERS: &str = include_str!("answers.txt");

pub fn new() -> WordsList<5> {
    WordsList::new_exclusive(
        Guessable::from_str(ALLOWED).unwrap(),
        Answers::from_str(ANSWERS).unwrap(),
    )
}
