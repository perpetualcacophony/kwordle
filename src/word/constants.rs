use crate::Word;

#[allow(non_snake_case)]
pub fn AMBER() -> Word<5> {
    unsafe { Word::from_str_unchecked("amber").expect("hard-coded word should be valid") }
}

#[allow(non_snake_case)]
pub fn SONAR() -> Word<5> {
    unsafe { Word::from_str_unchecked("sonar").expect("hard-coded word should be valid") }
}

#[allow(non_snake_case)]
pub fn MUMMY() -> Word<5> {
    unsafe { Word::from_str_unchecked("mummy").expect("hard-coded word should be valid") }
}

#[allow(non_snake_case)]
pub fn TUMMY() -> Word<5> {
    unsafe { Word::from_str_unchecked("tummy").expect("hard-coded word should be valid") }
}
