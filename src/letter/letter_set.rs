use std::{
    collections::BTreeSet,
    ops::{Deref, DerefMut},
};

use crate::Letter;

#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Hash)]
pub struct LetterSet(BTreeSet<Letter>);

impl LetterSet {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self.0.iter())
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

impl<'a> IntoIterator for &'a LetterSet {
    type Item = &'a Letter;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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

pub struct Iter<'a> {
    base: std::collections::btree_set::Iter<'a, Letter>,
}

impl<'a> Iter<'a> {
    fn new(base: std::collections::btree_set::Iter<'a, Letter>) -> Self {
        Self { base }
    }
}

impl<'a> Deref for Iter<'a> {
    type Target = std::collections::btree_set::Iter<'a, Letter>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'a> DerefMut for Iter<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Letter;

    fn next(&mut self) -> Option<Self::Item> {
        self.deref_mut().next()
    }
}
