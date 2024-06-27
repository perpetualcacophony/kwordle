use crate::{Guesses, Word};

pub struct Game<const WORD_LEN: usize> {
    answer: Word<WORD_LEN>,
    guesses: Guesses<WORD_LEN>,
}

impl<const WORD_LEN: usize> Game<WORD_LEN> {
    pub fn new(answer: Word<WORD_LEN>, max_guesses: impl Into<Option<usize>>) -> Self {
        Self {
            answer,
            guesses: Guesses::new(max_guesses),
        }
    }
}
