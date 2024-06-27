use crate::letter::Letter;

use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct LettersMap {
    pub(crate) hash_map: HashMap<Letter, usize>,
}

impl LettersMap {
    pub fn new() -> Self {
        HashMap::new().into()
    }

    pub fn count_letter(&self, letter: Letter) -> usize {
        self.hash_map.get(&letter).copied().unwrap_or_default()
    }

    pub fn contains_letter(&self, letter: Letter) -> bool {
        self.hash_map.contains_key(&letter)
    }

    pub fn insert(&mut self, letter: Letter) {
        self.hash_map.insert(letter, 1);
    }

    pub fn increment(&mut self, letter: Letter) {
        if let Some(count) = self.hash_map.get_mut(&letter) {
            *count += 1;
        } else {
            self.insert(letter)
        }
    }

    pub fn decrement(&mut self, letter: Letter) -> Option<usize> {
        if self.contains_letter(letter) {
            if self.count_letter(letter) == 1 {
                self.hash_map.remove(&letter);
                Some(0)
            } else {
                let count = self
                    .hash_map
                    .get_mut(&letter)
                    .expect("already checked that the map contains this letter");
                *count -= 1; // will never be 0
                Some(*count)
            }
        } else {
            None
        }
    }
}

impl From<HashMap<Letter, usize>> for LettersMap {
    fn from(value: HashMap<Letter, usize>) -> Self {
        Self { hash_map: value }
    }
}

impl FromIterator<Letter> for LettersMap {
    fn from_iter<T: IntoIterator<Item = Letter>>(iter: T) -> Self {
        let mut map = Self::new();

        for letter in iter {
            map.increment(letter);
        }

        map
    }
}
