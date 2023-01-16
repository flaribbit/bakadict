use crate::types;

fn open_database() -> rusqlite::Result<rusqlite::Connection> {
    let self_path = std::env::current_exe().unwrap();
    let parent_path = self_path.parent().unwrap();
    let database_path = "databases/jp.db";
    rusqlite::Connection::open(parent_path.join(database_path))
        .or_else(|_| rusqlite::Connection::open(database_path))
}

pub fn find_word(word: &str) -> rusqlite::Result<()> {
    let conn = open_database()?;
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
