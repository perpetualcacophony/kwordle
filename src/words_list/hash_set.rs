use std::collections::HashSet;

use crate::Word;

/// A basic implementor of [WordsList] based on a [HashSet].
#[derive(Debug, Clone)]
pub struct HashSetWordsList<const N: usize> {
    set: HashSet<Word<N>>,
}

impl<const N: usize> super::WordsList<N> for HashSetWordsList<N> {}

impl<const N: usize> super::WordsListCore<N> for HashSetWordsList<N> {
    fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    #[cfg(feature = "rand")]
    fn try_random<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Option<Word<N>> {
        use rand::seq::IteratorRandom;

        self.set.iter().choose(rng).copied()
    }

    fn collection_contains(&self, word: Word<N>) -> bool {
        self.set.contains(&word)
    }

    fn from_words<It: IntoIterator<Item = Word<N>>>(words: It) -> Self {
        Self {
            set: words.into_iter().collect(),
        }
    }
}
