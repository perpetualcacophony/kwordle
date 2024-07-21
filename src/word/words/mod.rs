use std::str::FromStr;

use crate::{letter::letters::ParseLettersError, Letters, Word};

pub fn parse_from_str<const N: usize>(s: &str) -> Result<Box<[Word<N>]>, ParseLettersError> {
    let words = s
        .lines()
        .map(Letters::from_str)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|letters| unsafe { Word::new_unchecked(letters) });

    Ok(words.collect())
}
