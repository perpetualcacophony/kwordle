use std::ops::{Index, IndexMut};

use crate::Letters;

use super::letter::Letter;

pub mod guesses;
pub use guesses::Guesses;

mod letter_state;
pub use letter_state::LetterState;
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde_derive",
    serde(
        bound = "crate::Letters<N, (Letter, LetterState)>: serde::Serialize + for<'a> serde::Deserialize<'a>"
    )
)]
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
    use super::{Guess, LetterState};

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

    impl<const N: usize> fmt::Test for Guess<N> {
        fn fmt(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
            for (_, state) in self.letters {
                fmt::Test::fmt(&state, f)?;
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
                .map(|l| (crate::Letter::A, l));

            Some(Self {
                letters: crate::Letters::from_iter(letters_states),
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
