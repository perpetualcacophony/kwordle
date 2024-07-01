pub mod adapter;
pub use adapter::Adapter;

use crate::Word;

use super::ParseWordsListError;

#[derive(Debug, Clone)]
pub struct Collection<const N: usize, I> {
    inner: I,
}

impl<const N: usize, A> Collection<N, A>
where
    A: adapter::Adapter<N>,
{
    fn new_unchecked(inner: A) -> Self {
        Self { inner }
    }

    pub fn try_new(inner: A) -> Option<Self> {
        if inner.is_empty() {
            None
        } else {
            Some(Self::new_unchecked(inner))
        }
    }

    pub fn from_words<It>(words: It) -> Option<Self>
    where
        It: IntoIterator<Item = A::Word>,
    {
        Self::try_new(A::from_words(words))
    }

    pub fn from_str(s: &str) -> Result<Self, ParseWordsListError>
    where
        A: Adapter<N, Word = crate::Word<N>>,
    {
        use crate::Letters;
        use std::str::FromStr;

        let words = s
            .lines()
            .map(Letters::from_str)
            .collect::<Result<Vec<Letters<N>>, _>>()?
            .into_iter()
            .map(Word::new_unchecked);

        Self::from_words(words).ok_or(ParseWordsListError::EmptyInput)
    }

    pub fn contains(&self, word: crate::Word<N>) -> bool {
        self.inner.contains(&word)
    }

    #[cfg(feature = "rand")]
    pub fn random_with<R>(&self, rng: &mut R) -> A::Word
    where
        R: rand::Rng,
    {
        self.inner
            .random(rng)
            .copied()
            .expect("inner collection should not be empty")
    }

    pub fn push(&mut self, word: A::Word) -> &A::Word {
        self.inner.push(word)
    }
}

pub type BorrowingCollection<'w, const N: usize, O: adapter::Owning<N>> =
    Collection<N, O::Borrowing<'w>>;

impl<const N: usize, O> Collection<N, O>
where
    O: adapter::Owning<N>,
{
    pub fn borrowing(&self) -> BorrowingCollection<'_, N, O> {
        Collection::new_unchecked(self.inner.borrowing())
    }

    pub fn iter<'a>(&'a self) -> <&'a O as IntoIterator>::IntoIter
    where
        &'a O: IntoIterator<Item = &'a Word<N>>,
    {
        self.inner.iter()
    }
}

impl<const N: usize, O> IntoIterator for Collection<N, O>
where
    O: adapter::Owning<N> + IntoIterator<Item = Word<N>>,
{
    type IntoIter = O::IntoIter;
    type Item = Word<N>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
