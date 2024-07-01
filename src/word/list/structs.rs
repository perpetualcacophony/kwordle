use super::WordsList;
use std::collections::HashSet;

pub struct HashSetWordsList<const WORD_LEN: usize> {
    collection: super::Collection<WORD_LEN, HashSet<crate::Word<WORD_LEN>>>,
}

impl<const WORD_LEN: usize> super::WordsList<WORD_LEN> for HashSetWordsList<WORD_LEN> {
    type Collection = std::collections::HashSet<crate::Word<WORD_LEN>>;

    fn collection(&self) -> &super::Collection<WORD_LEN, Self::Collection> {
        &self.collection
    }

    fn from_collection(collection: super::Collection<WORD_LEN, Self::Collection>) -> Self {
        Self { collection }
    }

    fn collection_mut(&mut self) -> &mut super::Collection<WORD_LEN, Self::Collection> {
        &mut self.collection
    }
}

impl<const N: usize> IntoIterator for HashSetWordsList<N> {
    type IntoIter = <<Self as WordsList<N>>::Collection as IntoIterator>::IntoIter;
    type Item = crate::Word<N>;

    fn into_iter(self) -> Self::IntoIter {
        self.collection.into_iter()
    }
}
