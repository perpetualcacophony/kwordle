use super::letter::Letter;
use crate::letter::LetterSet;

pub mod guesses;
pub use guesses::Guesses;

use crate::Array;

mod letter_state;
pub use letter_state::LetterState;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde_derive",
    serde(
        bound = "Array<(Letter, LetterState), N>: serde::Serialize + for<'a> serde::Deserialize<'a>"
    )
)]
pub struct Guess<const N: usize = 5> {
    array: Array<(Letter, LetterState), N>,
}

impl<const N: usize> Guess<N> {
    pub fn none_present(letters: Array<Letter, N>) -> Self {
        Self {
            array: Array::new(letters.map(|letter| (letter, LetterState::NotPresent))),
        }
    }

    pub fn letters(&self) -> std::array::IntoIter<Letter, N> {
        self.array.map(|(letter, _)| letter).into_iter()
    }

    pub fn states(&self) -> std::array::IntoIter<LetterState, N> {
        self.array.map(|(_, state)| state).into_iter()
    }

    pub fn is_correct(self) -> bool {
        self.states().all(LetterState::is_correct)
    }

    pub fn get(self, index: usize) -> Option<(Letter, LetterState)> {
        self.array.get(index).copied()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<(Letter, &mut LetterState)> {
        self.array.get_mut(index).map(|tup| (tup.0, &mut tup.1))
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, N> {
        IterMut::new(self)
    }

    pub fn unused_letters(self) -> LetterSet {
        let mut set = crate::letter::alphabet_set();
        self.unused_letters_with(&mut set);
        set
    }

    fn unused_letters_with(self, set: &mut LetterSet) {
        for (letter, _) in self {
            set.remove(&letter);
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde_derive", derive(serde::Serialize, serde::Deserialize))]
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

type IntoIterBase<const N: usize> = std::array::IntoIter<(Letter, LetterState), N>;

pub struct IntoIter<const N: usize> {
    base: IntoIterBase<N>,
}

impl<const N: usize> IntoIter<N> {
    fn new(guess: Guess<N>) -> Self {
        Self {
            base: guess.array.into_iter(),
        }
    }
}

impl<const N: usize> Iterator for IntoIter<N> {
    type Item = (Letter, LetterState);

    fn next(&mut self) -> Option<Self::Item> {
        self.base.next()
    }
}

impl<const N: usize> IntoIterator for Guess<N> {
    type IntoIter = IntoIter<N>;
    type Item = (Letter, LetterState);

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

type IterMutBase<'a> = std::iter::Map<
    std::slice::IterMut<'a, (Letter, LetterState)>,
    fn(&mut (Letter, LetterState)) -> (Letter, &mut LetterState),
>;

pub struct IterMut<'g, const N: usize> {
    base: IterMutBase<'g>,
}

impl<'g, const N: usize> IterMut<'g, N> {
    fn new(guess: &'g mut Guess<N>) -> Self {
        Self {
            base: guess.array.iter_mut().map(|tup| (tup.0, &mut tup.1)),
        }
    }
}

impl<'g, const N: usize> Iterator for IterMut<'g, N> {
    type Item = (Letter, &'g mut LetterState);

    fn next(&mut self) -> Option<Self::Item> {
        self.base.next()
    }
}

impl<'g, const N: usize> IntoIterator for &'g mut Guess<N> {
    type Item = (Letter, &'g mut LetterState);
    type IntoIter = IterMut<'g, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::LetterState;

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

    impl fmt::Test for Vec<LetterState> {
        fn fmt(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
            for state in self {
                fmt::Test::fmt(state, f)?;
            }

            Ok(())
        }

        fn from_str(s: &str) -> Option<Self>
        where
            Self: Sized,
        {
            Some(
                s.chars()
                    .filter_map(|ch| LetterState::from_str(&ch.to_string()))
                    .collect(),
            )
        }
    }

    macro_rules! string_match {
        ($($word:ident, $guess:ident => $result:literal;)+) => {
            paste::paste! {
                $(
                    #[test]
                    fn [<$word _ $guess>]() {
                        use fmt::Test;

                        unsafe {
                            let word: crate::Word<5> = crate::Word::from_str_unchecked(&stringify!($word)).unwrap();
                            let guess = word.guess(crate::Word::from_str_unchecked(&stringify!($guess)).unwrap());
                            pretty_assertions::assert_eq!(
                                guess.states().collect::<Vec<_>>().to_string(), $result
                            )
                        }
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
