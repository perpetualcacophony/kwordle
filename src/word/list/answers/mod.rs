use crate::{word::words::Words, Word};

#[derive(Debug, Clone)]
pub struct Answers<const N: usize> {
    base: Words<N>,
}

impl<const N: usize> Answers<N> {
    fn new_unchecked(words: Words<N>) -> Self {
        Self { base: words }
    }

    fn try_new(words: Words<N>) -> Option<Self> {
        if words.is_empty() {
            None
        } else {
            Some(Self::new_unchecked(words))
        }
    }

    pub fn try_from_iter<I>(iter: I) -> Option<Self>
    where
        I: IntoIterator<Item = Word<N>>,
    {
        let base = iter.into_iter().collect();
        Self::try_new(base)
    }
}

impl<const N: usize> IntoIterator for Answers<N> {
    type Item = Word<N>;
    type IntoIter = <Words<N> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.base.into_iter()
    }
}

impl<const N: usize> Answers<N> {
    fn append_to_guessable(&self, guessable: &mut super::guessable::Guessable<N>) {
        guessable.extend(self.clone())
    }

    #[cfg(feature = "rand")]
    pub fn random_with<R>(&self, rng: &mut R) -> Word<N>
    where
        R: rand::Rng,
    {
        use rand::seq::SliceRandom;

        self.base
            .as_slice()
            .choose(rng)
            .copied()
            .expect("Answers should not be empty")
    }

    #[cfg(feature = "rand_full")]
    pub fn random(&self) -> Word<N> {
        self.random_with(&mut rand::thread_rng())
    }
}
