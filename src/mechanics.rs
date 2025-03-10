use crate::utils;
use crate::utils::GraduationLevel;

pub enum Number {
    एकवचनम्,
    द्विवचनम्,
    बहुवचनम्
}
pub enum Person {
    FIRST,
    SECOND,
    THIRD
}
pub enum Tense {
    लट्,
    लङ्,
    लृट्
    // TODO: Add more tenses
}
pub enum Voice {
    ACTIVE,
    MIDDLE,
    PASSIVE
}
pub enum Mood {
    INDICATIVE,
    OPTATIVE,
    IMPERATIVE,
    CONDITIONAL
}

pub trait Class {
    fn stem(&self, tense: Tense) -> String;
    fn conjugate(&self, number: Number, person: Person, tense: Tense, voice: Option<Voice>, mood: Option<Mood>) -> String;
    fn participle(&self, tense: Tense, voice: Voice, gerundive: bool) -> String;
}

pub struct भ्वादि {
    root: String
}

impl भ्वादि {
    pub fn new(root: &str) -> Self {
        भ्वादि { root: String::from(root) }
    }
}

impl Class for भ्वादि {
    fn stem(&self, tense: &Tense) -> String {
        let strengthened = utils::vowel_graduation(self.root.clone(), GraduationLevel::GUNA);

        let mut tokens = strengthened.chars().collect::<Vec<char>>();
        if tokens[tokens.len() - 1] == '्' {
            tokens.remove(tokens.len() - 1);
        } else {
            tokens.push('ा');
        }
        match tense {
            Tense::लट् => {
                tokens.iter().map(|c| *c).collect::<String>()
            },
            Tense::लङ् => {
                tokens.insert(0, 'अ');
                tokens.iter().map(|c| *c).collect::<String>()
            },
            Tense::लृट् => { todo!() }
        }
    }

    fn conjugate(&self, number: Number, person: Person, tense: Tense, voice: Option<Voice>, mood: Option<Mood>) -> String {
        let mut stem = self.stem(&tense);
        let person_marker = match &number {
            Number::एकवचनम् => {
                match &tense {
                    Tense::लट् | Tense::लृट् => {
                        match &person {
                            Person::FIRST => "ति",
                            Person::SECOND => "सि",
                            Person::THIRD => "ामि"
                        }
                    },
                    Tense::लङ् => {
                        match &person {
                            Person::FIRST => "ति",
                            Person::SECOND => "सि",
                            Person::THIRD => "ामि"
                        }
                    },
                }
            },
            Number::द्विवचनम् => { "" },
            Number::बहुवचनम् => { "" }
        };

        stem.push_str(person_marker);
        stem
    }

    fn participle(&self, tense: Tense, voice: Voice, gerundive: bool) -> String {
        todo!()
    }
}