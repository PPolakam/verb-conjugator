use crate::mechanics::Class;
use crate::utils::{vowel_graduation, GraduationLevel};

mod mechanics;
mod utils;

fn main() {
    let kriyapada = mechanics::भ्वादि::new("भृ");
    let stem = kriyapada.stem(mechanics::Tense::लङ्);
    println!("{stem}");
}


