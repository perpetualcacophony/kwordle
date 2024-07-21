use std::str::FromStr;

use serde::Deserialize;

use crate::{Letters, Word, WordsList};

#[derive(Clone, Copy, Debug)]
pub struct WordDeserializer<'list, const N: usize = 5> {
    list: &'list WordsList<N>,
}

impl<'list, const N: usize> WordDeserializer<'list, N> {
    pub fn new(list: &'list WordsList<N>) -> Self {
        Self { list }
    }
}

impl<'de, 'list, const N: usize> serde::de::DeserializeSeed<'de> for WordDeserializer<'list, N> {
    type Value = Word<N>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;

        let letters =
            Letters::from_str(s).map_err(|err| serde::de::Error::custom(err.to_string()))?;

        let word = Word::from_letters(self.list, letters);

        word.map_err(|err| serde::de::Error::custom(err.to_string()))
    }
}
