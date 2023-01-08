pub struct WordItem {
    pub word: String,
    pub pronounce: String,
    pub accent: String,
    pub explain: String,
    pub sentences: String,
}

use std::fmt;

impl fmt::Display for WordItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{} {}\n{}\n{}",
            self.word, self.pronounce, self.accent, self.explain, self.sentences
        )
    }
}

#[test]
fn test_display_word_item() {
    let word_item = WordItem {
        word: "".to_owned(),
        pronounce: "".to_owned(),
        accent: "".to_owned(),
        explain: "".to_owned(),
        sentences: "".to_owned(),
    };
    println!("{word_item}")
}
