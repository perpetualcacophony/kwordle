use crate::letter::Letter;

/// Represents the status of an individual letter in a guess.
///
/// In classic wordle, this is displayed by the color of each letter tile.
/// In a more general sense, this is the primary means by which information
/// about the puzzle's answer is communicated to the player.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
pub enum LetterState {
    #[default]
    NotPresent,
    WrongPlace,
    Correct,
}

impl LetterState {
    pub fn not_present(letter: Letter) -> (Letter, Self) {
        (letter, Self::NotPresent)
    }

    pub fn is_correct(self) -> bool {
        self == Self::Correct
    }
}
