use crate::guess::Guess;

pub struct Guesses<const N: usize> {
    max: Option<usize>,
    vec: Vec<Guess<N>>,
}

pub type Classic = Guesses<5>;

impl<const N: usize> Guesses<N> {
    pub fn new(max: impl Into<Option<usize>>) -> Self {
        Self {
            max: max.into(),
            vec: Vec::new(),
        }
    }

    pub fn latest(&self) -> Option<Guess<N>> {
        self.vec.last().copied()
    }

    pub fn classic() -> Classic {
        Classic::default()
    }

    pub fn count(&self) -> usize {
        self.vec.len()
    }

    pub fn max_reached(&self) -> bool {
        Some(self.count()) == self.max
    }

    pub fn push(&mut self, guess: Guess<N>) -> bool {
        if self.max_reached() {
            false
        } else {
            self.vec.push(guess);
            true
        }
    }

    pub fn as_slice(&self) -> &[Guess<N>] {
        &self.vec
    }

    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl Default for Classic {
    fn default() -> Self {
        Self::new(6)
    }
}

impl<'a, const N: usize> IntoIterator for &'a Guesses<N> {
    type Item = &'a Guess<N>;
    type IntoIter = std::slice::Iter<'a, Guess<N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().into_iter()
    }
}
