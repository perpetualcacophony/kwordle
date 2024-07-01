mod hash_set;

pub trait Adapter<const WORD_LEN: usize>: crate::Sealed
where
    Self: FromIterator<Self::Word> + IntoIterator<Item = Self::Word>,
{
    type Word: Copy + PartialEq<crate::Word<WORD_LEN>>;

    fn is_empty(&self) -> bool;

    fn from_words<It>(words: It) -> Self
    where
        It: IntoIterator<Item = Self::Word>,
    {
        Self::from_iter(words)
    }

    #[cfg(feature = "rand")]
    fn random<R>(&self, rng: &mut R) -> Option<&Self::Word>
    where
        R: rand::Rng;

    #[cfg(feature = "rand")]
    fn iterator_random<'a, R>(&'a self, rng: &mut R) -> Option<&'a Self::Word>
    where
        R: rand::Rng,
        &'a Self: IntoIterator<Item = &'a Self::Word>,
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

    fn push(&mut self, word: Self::Word) -> &Self::Word;
}

pub trait Owning<const N: usize>
where
    Self: Adapter<N, Word = crate::Word<N>>,
{
    type Borrowing<'w>: Borrowing<'w, N, Self>
    where
        Self: 'w;

    fn borrowing<'w>(&'w self) -> Self::Borrowing<'w>;

    fn iter<'a>(&'a self) -> <&'a Self as IntoIterator>::IntoIter
    where
        &'a Self: IntoIterator<Item = &'a Self::Word>,
    {
        self.into_iter()
    }
}

pub trait Borrowing<'w, const N: usize, O>
where
    Self: Adapter<N, Word = &'w crate::Word<N>>,
    O: Owning<N, Borrowing<'w> = Self> + 'w,
{
}
