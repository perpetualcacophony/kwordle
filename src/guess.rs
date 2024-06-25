use std::ops::{Index, IndexMut};

use super::letter::Letter;
use super::letter_state::LetterState;

#[derive(Copy, Clone, Debug)]
pub struct Guess<const N: usize> {
    letters: [(Letter, LetterState); N]
}

pub type Classic = Guess<5>;

impl<const N: usize> Guess<N> {
    pub fn none_present(letters: [Letter; N]) -> Self {
        Self { letters: letters.map(LetterState::not_present) }
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