pub struct HashSetWordsList<const WORD_LEN: usize> {
    core: super::core::HashSetCore<WORD_LEN>,
}

impl<const WORD_LEN: usize> super::WordsList<WORD_LEN> for HashSetWordsList<WORD_LEN> {
    type Collection = std::collections::HashSet<crate::Word<WORD_LEN>>;

    fn core(&self) -> &super::core::WordsListCore<Self::Collection, WORD_LEN> {
        &self.core
    }

    fn from_core(core: super::core::WordsListCore<Self::Collection, WORD_LEN>) -> Self {
        Self { core }
    }
}
