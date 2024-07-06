use std::ops::Index;

use super::letter::Letter;

pub mod guesses;
pub use guesses::Guesses;

use crate::Array;

mod letter_state;
pub use letter_state::LetterState;
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde_derive",
    serde(bound = "Array<LetterWithState, N>: serde::Serialize + for<'a> serde::Deserialize<'a>")
)]
pub struct Guess<const N: usize> {
    letters: Array<LetterWithState, N>,
}

impl<const N: usize> Guess<N> {
    pub fn none_present(letters: Array<Letter, N>) -> Self {
        Self {
            letters: Array::new(letters.map(LetterWithState::default)),
        }
    }

    pub fn is_correct(self) -> bool {
        self.into_iter().all(LetterWithState::is_correct)
    }

    pub fn get(self, index: usize) -> Option<LetterWithState> {
        self.letters.get(index).copied()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut LetterWithState> {
        self.letters.get_mut(index)
    }

    pub fn as_slice(&self) -> &[LetterWithState] {
        self.letters.as_slice()
    }

    pub fn as_mut_slice<'g>(&'g mut self) -> &'g mut [LetterWithState] {
        self.letters.as_mut_slice()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, N> {
        self.into_iter()
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct LetterWithState {
    letter: Letter,
    state: LetterState,
}

impl LetterWithState {
    pub fn new(letter: Letter, state: LetterState) -> Self {
        Self { letter, state }
    }

    pub fn default(letter: Letter) -> Self {
        Self::new(letter, LetterState::default())
    }

    pub fn letter(self) -> Letter {
        self.letter
    }

    pub fn state(self) -> LetterState {
        self.state
    }

    pub fn set_state(&mut self, new: LetterState) {
        self.state = new
    }

    pub fn is_correct(self) -> bool {
        self.state().is_correct()
    }
}

impl<const N: usize> Index<usize> for Guess<N> {
    type Output = LetterWithState;

    fn index(&self, index: usize) -> &Self::Output {
        self.letters.index(index)
    }
}

pub struct IntoIter<const N: usize> {
    base: std::array::IntoIter<LetterWithState, N>,
}

impl<const N: usize> IntoIter<N> {
    fn new(guess: Guess<N>) -> Self {
        Self {
            base: guess.letters.into_iter(),
        }
    }
}

impl<const N: usize> Iterator for IntoIter<N> {
    type Item = LetterWithState;

    fn next(&mut self) -> Option<Self::Item> {
        self.base.next()
    }
}

impl<const N: usize> IntoIterator for Guess<N> {
    type IntoIter = IntoIter<N>;
    type Item = LetterWithState;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

pub struct IterMut<'g, const N: usize> {
    base: std::slice::IterMut<'g, LetterWithState>,
}

impl<'g, const N: usize> IterMut<'g, N> {
    fn new(guess: &'g mut Guess<N>) -> Self {
        Self {
            base: guess.letters.iter_mut(),
        }
    }
}

impl<'g, const N: usize> Iterator for IterMut<'g, N> {
    type Item = &'g mut LetterWithState;

    fn next(&mut self) -> Option<Self::Item> {
        self.base.next()
    }
}

impl<'g, const N: usize> IntoIterator for &'g mut Guess<N> {
    type Item = &'g mut LetterWithState;
    type IntoIter = IterMut<'g, N>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{Array, Guess, LetterState};

    mod fmt {
        pub trait Test {
            fn fmt(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result;

            fn to_string(&self) -> String {
                let mut s = String::new();
                self.fmt(&mut s).unwrap();
                s
            }

            fn from_str(s: &str) -> Option<Self>
            where
                Self: Sized;
        }
    }

    impl fmt::Test for LetterState {
        fn fmt(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
            f.write_char(match self {
                Self::Correct => 'O',
                Self::WrongPlace => 'o',
                Self::NotPresent => '.',
            })
        }

        fn from_str(s: &str) -> Option<Self>
        where
            Self: Sized,
        {
            match s {
                "O" => Some(Self::Correct),
                "o" => Some(Self::WrongPlace),
                "." => Some(Self::NotPresent),
                _ => None,
            }
        }
    }

    impl<const N: usize> fmt::Test for Guess<N>
    where
        Guess<N>: Copy,
    {
        fn fmt(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
            for letter in *self {
                fmt::Test::fmt(&letter.state(), f)?;
            }

            Ok(())
        }

        fn from_str(s: &str) -> Option<Self>
        where
            Self: Sized,
        {
            let letters_states = s
                .chars()
                .filter_map(|ch| LetterState::from_str(&ch.to_string()))
                .map(|l| super::LetterWithState::new(crate::Letter::A, l));

            Some(Self {
                letters: Array::from_iter(letters_states).ok()?,
            })
        }
    }

    macro_rules! string_match {
        ($($word:ident, $guess:ident => $result:literal;)+) => {
            paste::paste! {
                $(
                    #[test]
                    fn [<$word _ $guess>]() {
                        use fmt::Test;

                        let word: crate::Word<5> = crate::Word::from_str_unchecked(&stringify!($word)).unwrap();
                        let guess = word.guess_word(crate::Word::from_str_unchecked(&stringify!($guess)).unwrap());
                        pretty_assertions::assert_eq!(
                            guess.to_string(), $result
                        )
                    }
                )+
            }
        };
    }

    string_match! {
        amber, amber => "OOOOO";
        amber, arbor => "O.O.O";
        amber, handy => ".o...";
        addra, opals => "..o..";
        mummy, tummy => ".OOOO";

        // these tests were made by annie!!
        vital, audio => "o..o.";
        scene, eager => "o..o.";
        today, level => ".....";
        phone, crown => "..O.o";
        royal, newly => "...oo";
        baker, dying => ".....";
        level, topic => ".....";
        blind, began => "O...o";
        movie, storm => "..o.o";
        spend, super => "O.oo.";
        still, worth => "...o.";
        build, usage => "o....";
        badly, alive => "oo...";
        harry, count => ".....";
        split, house => "...o.";
        quite, trust => "o.o..";
        flash, death => "..O.O";
        peter, crime => ".o..o";
        title, china => "..o..";
        these, smith => "o..oo";
        sport, lying => ".....";
        solve, shoot => "O.o..";
        prior, whole => "..o..";
        maybe, fruit => ".....";
        event, dealt => ".o..O";
    }
}
