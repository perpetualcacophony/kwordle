use std::{
    collections::BTreeSet,
    ops::{Deref, DerefMut},
};

use crate::Letter;

pub struct LetterSet(BTreeSet<Letter>);

impl LetterSet {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }
}

impl Deref for LetterSet {
    type Target = BTreeSet<Letter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LetterSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<Letter> for LetterSet {
    fn from_iter<T: IntoIterator<Item = Letter>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for LetterSet {
    type Item = Letter;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.0.into_iter())
    }
}

pub struct IntoIter<Base = std::collections::btree_set::IntoIter<Letter>> {
    base: Base,
}

impl<Base> IntoIter<Base> {
    fn new(base: Base) -> Self {
        Self { base }
    }
}

impl<Base: Iterator<Item = Letter>> Iterator for IntoIter<Base> {
    type Item = Letter;

    fn next(&mut self) -> Option<Self::Item> {
        self.base.next()
    }
}

impl Deref for IntoIter {
    type Target = std::collections::btree_set::IntoIter<Letter>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for IntoIter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
