use crate::utils::{vowel_graduation, GraduationLevel};

mod mechanics;
mod utils;

fn main() {
    for root in vec!["कृ", "अस्‌", "गम्‌", "दा", "लिख्‌", "नी"] {
        let res = vowel_graduation(String::from(root), GraduationLevel::GUNA);
        println!("{}", res);
    }
}


