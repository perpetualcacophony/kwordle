use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Array<T, const N: usize> {
    array: [T; N],
}

impl<T, const N: usize> Array<T, N> {
    pub fn new(array: [T; N]) -> Self {
        Self { array }
    }

    pub fn from_iter<I>(iter: I) -> Result<Self, LengthError>
    where
        I: IntoIterator<Item = T>,
    {
        let vec = Vec::from_iter(iter);
        Self::try_from(vec)
    }
}

impl<T, const N: usize> Deref for Array<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl<T, const N: usize> DerefMut for Array<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.array
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LengthError {
    expected: usize,
    got: usize,
}

impl LengthError {
    pub fn new(expected: usize, got: usize) -> Self {
        Self { expected, got }
    }

    pub fn from_slices<T, U>(expected: &[T], got: &[U]) -> Self {
        Self::new(expected.len(), got.len())
    }
}

impl std::fmt::Display for LengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected {} items, got {}", self.expected, self.got)
    }
}

impl<T, const N: usize> TryFrom<Vec<T>> for Array<T, N> {
    type Error = LengthError;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        let array: [T; N] = value
            .try_into()
            .map_err(|vec: Vec<T>| LengthError::new(N, vec.len()))?;
        Ok(Self { array })
    }
}

impl<T, const N: usize> IntoIterator for Array<T, N> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}
