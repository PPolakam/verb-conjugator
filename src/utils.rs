use std::collections::HashMap;

pub enum GraduationLevel {
    GUNA,
    VRDDHI
}

fn is_vowel(token: &str) -> bool {
    let vowels = vec![
        "अ", "आ", "इ", "ई", "उ", "ऊ", "ऋ", "ॠ", "ऌ", "ए", "ऐ", "ओ", "औ",
        "ा", "ि", "ी", "ु", "ू", "ृ", "ॄ", "े", "ै", "ो", "ौ"
    ];
    vowels.contains(&token)
}

fn graduate(vowel: &str, level: i32) -> String {
    let graduation_map: HashMap<&str, Vec<&str>> = HashMap::from([
        ("ृ", vec!["र्", "ार्"]),
        ("ॢ", vec!["ल्", "ाल्"]),
        ("ि", vec!["े", "ै"]),
        ("ी", vec!["े", "ै"]),
        ("ु", vec!["ो", "ौ"]),
        ("ू", vec!["ो", "ौ"]),
        ("ा", vec!["ा", "ा"]),
        ("ऋ", vec!["र्", "आर्"]),
        ("ऌ", vec!["ल्", "आल्"]),
        ("इ", vec!["ए", "ऐ"]),
        ("ई", vec!["ए", "ऐ"]),
        ("उ", vec!["ओ", "औ"]),
        ("ऊ", vec!["ओ", "औ"]),
        ("अ", vec!["आ", "आ"]),
        ("आ", vec!["आ", "आ"])
    ]);
    if graduation_map.contains_key(vowel) {
        graduation_map[vowel][level as usize].to_string()
    } else {
        vowel.to_string()
    }
}

pub fn vowel_graduation(string: String, level: GraduationLevel) -> String {
    let tokens = string.split("");

    let mut result = String::new();

    let mut graduated = false;

    for token in tokens {
        if is_vowel(token) {
            result.push_str(
                graduate(
                    token,
                    match level {
                        GraduationLevel::GUNA => 0,
                        GraduationLevel::VRDDHI => 1
                    }
                ).as_str()
            );
            graduated = true;
        } else {
            result.push_str(token);
        }
    }

    if !graduated {
        let mut intr = result.chars().map(|c| String::from(c)).collect::<Vec<String>>();
        let mut t = graduate(intr[0].as_str(), 0).to_string();
        intr.remove(0);
        t.push_str(&intr.join(""));
        t
    } else {
        result
    }
}