use super::letter::Letter;

#[derive(Copy, Clone, Debug)]
pub struct Letters<const N: usize, L = Letter> {
    array: [L; N]
}

impl<const N: usize> Letters<N> {
    pub fn new(array: [Letter; N]) -> Self {
        Self { array }
    }
}

impl<const N: usize, L> Letters<N, L> {
    pub fn map<U>(self, f: impl FnMut(L) -> U) -> Letters<N, U> {
        let array = self.array.map(f);
        Letters { array }
    }
}

pub struct Iter<'a> {
    slice: &'a [Letter],
    index: usize,
}

impl<'a> Iter<'a> {
    fn new(slice: &'a [Letter]) -> Self {
        Self {
            slice,
            index: 0
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Letter;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.slice.get(self.index);
        self.index += 1;
        item
    }
}

impl<'a, const N: usize> IntoIterator for &'a Letters<N> {
    type IntoIter = Iter<'a>;
    type Item = &'a Letter;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(&self.array)
    }
}

pub struct IntoIter<const N: usize> {
    index: usize,
    array: [Letter; N],
}

impl<const N: usize> IntoIter<N> {
    fn new(array: [Letter; N]) -> Self {
        Self {
            index: 0,
            array
        }
    }
}

impl<const N: usize> Iterator for IntoIter<N> {
    type Item = Letter;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.array.get(self.index).copied();
        self.index += 1;
        item
    }
}

impl<const N: usize> IntoIterator for Letters<N> {
    type IntoIter = IntoIter<N>;
    type Item = Letter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.array)
    }
}

pub type Standard = Letters<5>;

pub trait CountLetters {
    fn count_letters(&self) -> usize;

    fn ensure_letters(&self, other: &impl CountLetters) -> Result<(), CountLettersError> {
        if self.count_letters() == other.count_letters() {
            Ok(())
        } else {
            Err(CountLettersError { expected: self.count_letters(), got: other.count_letters() })
        }
    }
}

pub struct CountLettersError {
    expected: usize,
    got: usize
}