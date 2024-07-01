use super::Adapter;
use crate::Word;
use std::collections::HashSet;

impl<const N: usize> crate::Sealed for HashSet<Word<N>> {}
impl<const N: usize> super::Owning<N> for HashSet<Word<N>> {
    type Borrowing<'w> = HashSet<&'w Word<N>>;

    fn borrowing<'w>(&'w self) -> Self::Borrowing<'w> {
        self.into_iter().collect()
    }
}

impl<'a, const WORD_LEN: usize> Adapter<WORD_LEN> for HashSet<Word<WORD_LEN>> {
    type Word = Word<WORD_LEN>;

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    #[cfg(feature = "rand")]
    fn random<R>(&self, rng: &mut R) -> Option<&crate::Word<WORD_LEN>>
    where
        R: rand::Rng,
    {
        self.iterator_random(rng)
    }

    fn contains(&self, word: &crate::Word<WORD_LEN>) -> bool {
        self.contains(word)
    }

    fn push(&mut self, word: Self::Word) -> &Self::Word {
        self.insert(word);
        self.get(&word).unwrap()
    }
}

impl<'w, const N: usize> crate::Sealed for HashSet<&'w Word<N>> {}
impl<'w, const N: usize> super::Borrowing<'w, N, HashSet<Word<N>>> for HashSet<&'w Word<N>> {}

impl<'w, const N: usize> Adapter<N> for HashSet<&'w Word<N>> {
    type Word = &'w Word<N>;

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn contains(&self, word: &crate::Word<N>) -> bool {
        self.contains(word)
    }

    fn random<R>(&self, rng: &mut R) -> Option<&Self::Word>
    where
        R: rand::Rng,
    {
        self.iterator_random(rng)
    }

    fn push(&mut self, word: Self::Word) -> &Self::Word {
        self.insert(word);
        self.get(&word).unwrap()
    }
}
