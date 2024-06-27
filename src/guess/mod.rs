use std::ops::{Index, IndexMut};

use crate::Letters;

use super::letter::Letter;

pub mod guesses;
pub use guesses::Guesses;

mod letter_state;
pub use letter_state::LetterState;

#[derive(Copy, Clone, Debug)]
pub struct Guess<const N: usize> {
    letters: Letters<N, (Letter, LetterState)>,
}

pub type Classic = Guess<5>;

impl<const N: usize> Guess<N> {
    pub fn none_present(letters: Letters<N>) -> Self {
        Self {
            letters: letters.map(LetterState::not_present),
        }
    }

    pub fn set_state(&mut self, index: usize, state: LetterState) {
        self.index_mut(index).1 = state;
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, (Letter, LetterState)> {
        self.letters.iter_mut()
    }
}

impl<const N: usize> Index<usize> for Guess<N> {
    type Output = (Letter, LetterState);

    fn index(&self, index: usize) -> &Self::Output {
        self.letters.index(index)
    }
}

impl<const N: usize> IndexMut<usize> for Guess<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.letters.index_mut(index)
    }
}

impl<const N: usize> IntoIterator for Guess<N> {
    type IntoIter = std::array::IntoIter<Self::Item, N>;
    type Item = (Letter, LetterState);

    fn into_iter(self) -> Self::IntoIter {
        self.letters.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::LetterState;

    use super::Guess;

    trait TestFormat: Sized {
        fn fmt_test(&self, s: &mut String);

        fn to_test_fmt(&self) -> String {
            let mut s = String::new();
            self.fmt_test(&mut s);
            s
        }

        fn from_test_fmt(s: &str) -> Option<Self>;
    }

    impl TestFormat for LetterState {
        fn fmt_test(&self, s: &mut String) {
            s.push_str(match self {
                Self::Correct => "O",
                Self::WrongPlace => "o",
                Self::NotPresent => ".",
            })
        }

        fn from_test_fmt(s: &str) -> Option<Self> {
            match s {
                "O" => Some(Self::Correct),
                "o" => Some(Self::WrongPlace),
                "." => Some(Self::NotPresent),
                _ => None,
            }
        }
    }
}
