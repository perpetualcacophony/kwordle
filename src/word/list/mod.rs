mod collection;
mod core;
pub mod structs;

pub trait WordsList<const WORD_LEN: usize>
where
    Self: core::WordsListCore<WORD_LEN>,
{
}
