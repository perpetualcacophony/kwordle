#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct WordValidity {
    inner: WordValidityInner,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
enum WordValidityInner {
    #[default]
    Guess,

    GuessAndAnswer,
}

impl WordValidity {
    pub fn guess() -> Self {
        Self {
            inner: WordValidityInner::Guess,
        }
    }

    pub fn answer() -> Self {
        Self {
            inner: WordValidityInner::GuessAndAnswer,
        }
    }

    pub fn is_guess(self) -> bool {
        true
    }

    pub fn is_answer(self) -> bool {
        self.inner == WordValidityInner::GuessAndAnswer
    }
}
