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
            "{} {} {}\n\n",
            Paint::new(&self.word).bold(),
            Paint::green(&self.pronounce),
            Paint::green(&self.accent),
        )?;
        writeln!(
            f,
            "📘 {}\n{}{}\n",
            Paint::new("Definitions").underline(),
            Paint::blue("【".to_owned() + &self.type_ + "】"),
            self.explain,
        )?;
        if self.sentences.len() > 0 {
            writeln!(f, "📒 {}", Paint::new("Examples").underline(),)?;
            for (i, sentence) in self.sentences.split('\n').enumerate() {
                if i % 2 == 0 {
                    writeln!(f, "- {}", sentence)?;
                } else {
                    writeln!(f, "  {}", Paint::new(sentence).dimmed())?;
                };
            }
        }
        Ok(())
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
        sentences: "蛙が鳴いている
青蛙在叫
彼には何を言っても蛙の面に水だ。
跟他说什么也不顶用。"
            .to_owned(),
    };
    println!("{word_item}")
}
