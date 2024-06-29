use crate::{Guesses, Word, WordsList};

pub struct Game<List, const WORD_LEN: usize> {
    answer: Word<WORD_LEN>,
    words_list: List,
    guesses: Guesses<WORD_LEN>,
}

impl<List, const WORD_LEN: usize> Game<List, WORD_LEN>
where
    List: WordsList<WORD_LEN>,
{
    pub fn new(
        answer: Word<WORD_LEN>,
        words_list: List,
        max_guesses: impl Into<Option<usize>>,
    ) -> Self {
        Self {
            answer,
            words_list,
            guesses: Guesses::new(max_guesses),
        }
    }

    pub fn guess_str(&self, s: &str) -> Result<crate::Guess<WORD_LEN>, GameError> {
        let word = Word::from_str(&self.words_list, s).map_err(GameError::InvalidWord)?;

        Ok(self.answer.guess_word(word))
    }
}

pub enum GameError {
    InvalidWord(crate::word::ParseWordError),
}
