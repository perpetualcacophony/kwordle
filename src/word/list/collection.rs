pub trait WordsListCollection<const WORD_LEN: usize>
where
    Self: FromIterator<crate::Word<WORD_LEN>>,
{
    fn from_words<W>(words: W) -> Self
    where
        W: IntoIterator<Item = crate::Word<WORD_LEN>>,
    {
        Self::from_iter(words)
    }
}

impl<const WORD_LEN: usize> WordsListCollection<WORD_LEN>
    for std::collections::HashSet<crate::Word<WORD_LEN>>
{
}
