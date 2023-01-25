use crate::types;

fn open_database() -> rusqlite::Result<rusqlite::Connection> {
    let self_path = std::env::current_exe().unwrap();
    let parent_path = self_path.parent().unwrap();
    let database_path = "databases/jp.db";
    #[allow(deprecated)]
    let data_path = std::env::home_dir().unwrap().join(".config/bakadict");
    rusqlite::Connection::open(parent_path.join(database_path))
        .or_else(|_| rusqlite::Connection::open(data_path.join(database_path)))
        .or_else(|_| rusqlite::Connection::open(database_path))
}

pub fn find_word(word: &str, reverse: bool) -> rusqlite::Result<()> {
    let conn = open_database()?;
    let mut sql = "SELECT * FROM word WHERE word LIKE '%'||?1||'%' OR pron LIKE '%'||?1||'%'
ORDER BY (
CASE
WHEN word=?1 OR pron=?1 THEN 1
WHEN word LIKE ?1||'%' OR pron LIKE ?1||'%' THEN 2
ELSE 3
END
)"
    .to_owned();
    if reverse {
        sql += " DESC";
    }
    let mut stmt = conn.prepare(&sql)?;
    let words = stmt.query_map((word,), |row| {
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
    find_word("かわく", false).unwrap();
}
