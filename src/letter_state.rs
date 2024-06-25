use crate::letter::Letter;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum LetterState {
    #[default]
    NotPresent,
    WrongPlace,
    Correct
}

impl LetterState {
    pub fn not_present(letter: Letter) -> (Letter, Self) {
        (letter, Self::NotPresent)
    }
}