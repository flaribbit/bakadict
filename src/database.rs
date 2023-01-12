use crate::types;

pub fn find_word(word: &str) -> rusqlite::Result<()> {
    let conn = rusqlite::Connection::open("databases/jp.db")?;
    let param = format!("%{word}%");
    let mut stmt = conn.prepare("SELECT * FROM word WHERE word LIKE ? OR pron LIKE ?")?;
    let words = stmt.query_map((&param, &param), |row| {
        Ok(types::WordItem {
            word: row.get(1)?,
            type_: row.get(2)?,
            pronounce: row.get(3)?,
            accent: row.get(4)?,
            explain: row.get(5)?,
            sentences: row.get(6)?,
        })
    })?;
    for word in words {
        println!("{}", word.unwrap());
    }
    Ok(())
}

#[test]
fn test_find_word() {
    find_word("かわく").unwrap();
}
