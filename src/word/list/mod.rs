mod error;

pub use error::ParseWordsListError;

pub mod answers;
pub mod guessable;

// TODO: should guessable really contain answers? think about fn `contains_guess` or something
#[derive(Debug, Clone, PartialEq)]
pub struct WordsList<const N: usize = 5> {
    pub guessable: guessable::Guessable<N>,
    pub answers: answers::Answers<N>,
}

impl<const N: usize> WordsList<N> {
    /// Constructs a new `WordsList` with a [`Guessable`]
    /// that **does not** contain the words in the provided [`Answers`].
    ///
    /// # Panics
    /// Panics if `guessable` does not entirely exclude `answers`.
    pub fn new_exclusive(
        mut guessable: guessable::Guessable<N>,
        answers: answers::Answers<N>,
    ) -> Self {
        assert!(guessable.excludes_answers(&answers));
        answers.append_to_guessable(&mut guessable);
        Self { guessable, answers }
    }

    /// Constructs a new `WordsList` with a [`Guessable`]
    /// that already contains the words of the provided [`Answers`].
    pub fn new_inclusive(guessable: guessable::Guessable<N>, answers: answers::Answers<N>) -> Self {
        assert!(guessable.includes_answers(&answers));
        Self { guessable, answers }
    }

    pub fn from_guessable(guessable: guessable::Guessable<N>) -> Self {
        let answers = answers::Answers::from_guessable(&guessable);
        Self::new_inclusive(guessable, answers)
    }

    pub fn new(guessable: guessable::Guessable<N>, answers: answers::Answers<N>) -> Self {
        let new = if guessable.includes_answers(&answers) {
            Self::new_inclusive
        } else {
            Self::new_exclusive
        };

        new(guessable, answers)
    }

    #[cfg(feature = "classic_words")]
    pub fn classic() -> WordsList<5> {
        crate::classic::words_list()
    }
}
