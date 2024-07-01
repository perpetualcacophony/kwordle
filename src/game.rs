use crate::{Guesses, Word, WordsList};

pub struct Game<const WORD_LEN: usize> {
    answer: Word<WORD_LEN>,
    words_list: WordsList<WORD_LEN>,
    guesses: Guesses<WORD_LEN>,
}

impl<const WORD_LEN: usize> Game<WORD_LEN> {
    pub fn new(
        answer: Word<WORD_LEN>,
        words_list: WordsList<WORD_LEN>,
        max_guesses: impl Into<Option<usize>>,
    ) -> Self {
        Self {
            answer,
            words_list,
            guesses: Guesses::new(max_guesses),
        }
    }

    pub fn guess_str(&mut self, s: &str) -> Result<crate::Guess<WORD_LEN>, GameError> {
        let word = Word::from_str(&self.words_list, s).map_err(GameError::InvalidWord)?;

        let guess = self.answer.guess_word(word);

        self.guesses.push(guess);

        Ok(guess)
    }
}

pub enum GameError {
    InvalidWord(crate::word::ParseWordError),
}
