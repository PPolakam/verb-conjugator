use std::collections::HashMap;

pub enum GraduationLevel {
    GUNA,
    VRDDHI
}

fn is_vowel(token: &str) -> bool {
    let vowels = vec!["ा", "ि", "ी", "ु", "ू", "ृ", "ॄ", "े", "ै", "ो", "ौ"];
    vowels.contains(&token)
}

fn graduate(vowel: &str, level: i32) -> &str {
    let graduation_map: HashMap<&str, Vec<&str>> = HashMap::from([
        ("ृ", vec!["र्", "ार्‌"]),
        ("ॢ", vec!["ल्‌", "ाल्‌"]),
        ("ि", vec!["े", "ै"]),
        ("ी", vec!["े", "ै"]),
        ("ु", vec!["ो", "ौ"]),
        ("ू", vec!["ो", "ौ"])
    ]);
    graduation_map[vowel][level as usize]
}

fn vowel_graduation(string: String, level: GraduationLevel) -> String {
    let tokens = string.split("");

    let mut result = String::new();

    for token in tokens {
        if is_vowel(token) {
            result.push_str(
                graduate(
                    token,
                    match level {
                        GraduationLevel::GUNA => 0,
                        GraduationLevel::VRDDHI => 1
                    }
                )
            );
        } else {
            result.push_str(token);
        }
    }

    todo!()
}