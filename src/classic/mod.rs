#[cfg(feature = "classic_words")]
mod words_list;

#[cfg(feature = "classic_words")]
pub use words_list::new as words_list;

pub type Word = crate::Word<5>;
pub type Guess = crate::Guess<5>;
