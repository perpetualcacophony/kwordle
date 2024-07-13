mod error;
pub use error::ParseLettersError;

pub type Letters<const N: usize> = crate::Array<crate::Letter, N>;

impl<const N: usize> std::str::FromStr for Letters<N> {
    type Err = ParseLettersError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(
            s.chars()
                .map(crate::Letter::try_from)
                .collect::<Result<Vec<crate::Letter>, _>>()
                .map_err(ParseLettersError::ParseLetter)?,
        )
        .map_err(ParseLettersError::from)
    }
}
