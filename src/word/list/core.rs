pub(crate) trait WordsListCore<const WORD_LEN: usize> {
    type Collection: super::collection::WordsListCollection<WORD_LEN>;

    fn collection(&self) -> &Self::Collection;
    fn from_collection(collection: Self::Collection) -> Self;
}
