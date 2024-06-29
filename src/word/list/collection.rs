pub trait WordsListCollection<const WORD_LEN: usize>
where
    Self: FromIterator<crate::Word<WORD_LEN>> + crate::Sealed,
{
    fn is_empty(&self) -> bool;

    fn from_words<W>(words: W) -> Self
    where
        W: IntoIterator<Item = crate::Word<WORD_LEN>>,
    {
        Self::from_iter(words)
    }

    #[cfg(feature = "rand")]
    fn random<R>(&self, rng: &mut R) -> Option<&crate::Word<WORD_LEN>>
    where
        R: rand::Rng;

    #[cfg(feature = "rand")]
    fn iterator_random<R>(&self, rng: &mut R) -> Option<&crate::Word<WORD_LEN>>
    where
        for<'a> &'a Self: IntoIterator<Item = &'a crate::Word<WORD_LEN>>,
        R: rand::Rng,
    {
        use rand::seq::IteratorRandom;

        self.into_iter().choose(rng)
    }

    #[allow(dead_code)]
    #[cfg(feature = "rand")]
    fn slice_random<R>(&self, rng: &mut R) -> Option<&crate::Word<WORD_LEN>>
    where
        Self: AsRef<[crate::Word<WORD_LEN>]>,
        R: rand::Rng,
    {
        use rand::seq::SliceRandom;

        self.as_ref().choose(rng)
    }

    fn contains(&self, word: &crate::Word<WORD_LEN>) -> bool;

    fn union(&self, other: &Self) -> Self;
}

impl<const N: usize> crate::Sealed for std::collections::HashSet<crate::Word<N>> {}

impl<const WORD_LEN: usize> WordsListCollection<WORD_LEN>
    for std::collections::HashSet<crate::Word<WORD_LEN>>
{
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

    fn union(&self, other: &Self) -> Self {
        self.union(other).copied().collect()
    }
}
