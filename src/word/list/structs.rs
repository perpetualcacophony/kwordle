pub struct HashSetWordsList<const WORD_LEN: usize> {
    hash_set: std::collections::HashSet<crate::Word<WORD_LEN>>,
}

impl<const WORD_LEN: usize> super::core::WordsListCore<WORD_LEN> for HashSetWordsList<WORD_LEN> {
    type Collection = std::collections::HashSet<crate::Word<WORD_LEN>>;

    fn collection(&self) -> &Self::Collection {
        &self.hash_set
    }

    fn from_collection(collection: Self::Collection) -> Self {
        Self {
            hash_set: collection,
        }
    }
}

impl<const WORD_LEN: usize> super::WordsList<WORD_LEN> for HashSetWordsList<WORD_LEN> {}
