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
        writeln!(f, "๐ {}", Paint::new("Definitions").underline())?;
        if !self.type_.is_empty() {
            write!(f, "{}", Paint::red(format!("ใ{}ใ", self.type_)))?;
        }
        writeln!(f, "{}\n", self.explain)?;
        if !self.sentences.is_empty() {
            writeln!(f, "๐ {}", Paint::new("Examples").underline(),)?;
            for (i, sentence) in self.sentences.split('\n').enumerate() {
                if i % 2 == 0 {
                    writeln!(f, "- {}", sentence)?;
                } else {
                    writeln!(f, "  {}", Paint::new(sentence).dimmed())?;
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[test]
fn test_display_word_item() {
    let word_item = WordItem {
        word: "่".to_owned(),
        pronounce: "ใใใ".to_owned(),
        accent: "โ".to_owned(),
        type_: "ๅ่ฏ".to_owned(),
        explain: "้่ใ".to_owned(),
        sentences: "่ใ้ณดใใฆใใ
้่ๅจๅซ
ๅฝผใซใฏไฝใ่จใฃใฆใ่ใฎ้ขใซๆฐดใ ใ
่ทไป่ฏดไปไนไนไธ้กถ็จใ"
            .to_owned(),
    };
    println!("{word_item}")
}
