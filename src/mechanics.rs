enum Number {
    एकवचनम्,
    द्विवचनम्,
    बहुवचनम्
}
enum Person {
    FIRST,
    SECOND,
    THIRD
}
enum Tense {
    लट्,
    लङ्,
    लृट
    // TODO: Add more tenses
}
enum Voice {
    ACTIVE,
    MIDDLE,
    PASSIVE
}
enum Mood {
    INDICATIVE,
    OPTATIVE,
    IMPERATIVE,
    CONDITIONAL
}

trait Class {
    fn stem() -> String;
    fn conjugate(number: Number, person: Person, tense: Tense, voice: Option<Voice>, mood: Option<Mood>) -> String;
    fn participle(tense: Tense, voice: Voice, gerundive: bool) -> String;
}

struct भ्वादि {
    root: String
}

impl भ्वादि {
    fn new(&self, root: String) -> Self {
        भ्वादि { root }
    }
}

impl Class for भ्वादि {
    fn stem() -> String {
        // strengthen vowel
        todo!()
    }

    fn conjugate(number: Number, person: Person, tense: Tense, voice: Option<Voice>, mood: Option<Mood>) -> String {
        todo!()
    }

    fn participle(tense: Tense, voice: Voice, gerundive: bool) -> String {
        todo!()
    }
}

mod utils {

}