#![cfg_attr(feature = "step", feature(step_trait))]

pub mod letter;

#[doc(inline)]
pub use letter::Letter;
pub use letter::Letters;

pub mod guess;

#[doc(inline)]
pub use guess::Guess;

#[doc(inline)]
pub use guess::Guesses;

#[doc(inline)]
pub use guess::LetterState;

pub mod word;

#[doc(inline)]
pub use word::Word;

pub mod game;
pub use game::Game;

#[doc(inline)]
pub use word::WordsList;

pub mod classic;

mod array;
use array::Array;
