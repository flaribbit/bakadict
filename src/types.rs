pub struct WordItem {
    pub word: String,
    pub pronounce: String,
    pub accent: String,
    pub type_: String,
    pub explain: String,
    pub sentences: String,
}

use std::fmt;

impl fmt::Display for WordItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use yansi::Paint;
        write!(
            f,
            "{}\n{} {}\n{} {}\n{}",
            Paint::new(&self.word).bold(),
            Paint::red(&self.pronounce),
            Paint::red(&self.accent),
            Paint::blue("【".to_owned() + &self.type_ + "】"),
            self.explain,
            Paint::new(&self.sentences).dimmed(),
        )
    }
}

#[test]
fn test_display_word_item() {
    let word_item = WordItem {
        word: "蛙".to_owned(),
        pronounce: "かえる".to_owned(),
        accent: "◎".to_owned(),
        type_: "名词".to_owned(),
        explain: "青蛙。".to_owned(),
        sentences: "彼には何を言っても蛙の面に水だ。\n跟他说什么也不顶用。".to_owned(),
    };
    println!("{word_item}")
}
