use std::str::FromStr;

use crate::Word;

use super::ParseWordsListError;

pub struct WordsListCore<C, const N: usize> {
    collection: C,
}

impl<C, const N: usize> WordsListCore<C, N>
where
    C: super::WordsListCollection<N>,
{
    fn from_collection(collection: C) -> Result<Self, ParseWordsListError> {
        if collection.is_empty() {
            return Err(ParseWordsListError::EmptyInput);
        }

        Ok(Self { collection })
    }

    pub fn from_words<W>(words: W) -> Result<Self, ParseWordsListError>
    where
        W: IntoIterator<Item = Word<N>>,
        C: FromIterator<Word<N>>,
    {
        Self::from_collection(C::from_iter(words))
    }

    pub fn from_str(s: &str) -> Result<Self, ParseWordsListError>
    where
        C: FromIterator<Word<N>>,
    {
        use crate::Letters;

        let words = s
            .lines()
            .map(Letters::from_str)
            .collect::<Result<Vec<Letters<N>>, _>>()?
            .into_iter()
            .map(Word::new_unchecked);

        Self::from_words(words)
    }

    #[cfg(feature = "rand")]
    pub fn random_with<R>(&self, rng: &mut R) -> Word<N>
    where
        R: rand::Rng,
    {
        self.collection
            .random(rng)
            .copied()
            .expect("must not be empty")
    }

    pub fn contains(&self, word: Word<N>) -> bool {
        self.collection.contains(&word)
    }
}

pub type HashSetCore<const N: usize> = WordsListCore<std::collections::HashSet<Word<N>>, N>;
