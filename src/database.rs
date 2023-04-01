use crate::types;
use std::{env, path::PathBuf};

fn open_database() -> rusqlite::Result<rusqlite::Connection> {
    let database_path = get_db_path();

    rusqlite::Connection::open(&database_path)
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

fn get_db_path() -> PathBuf {
    match env::var("BAKADICT_JPDB_PATH") {
        Ok(val) => {
            if val.is_empty() {
                panic!("ERROR: BAKADICT_JPDB_PATH is null.")
            }

            val.into()
        }
        Err(e) => panic!("{e}"),
    }
}

mod test {
    use super::*;

    #[test]
    fn test_find_word() {
        find_word("かわく", false).unwrap();
    }

    #[test]
    fn test_get_db_path() {
        use std::env;

        env::set_var("BAKADICT_JPDB_PATH", "/test/jp.db");

        assert_eq!("/test/jp.db", get_db_path());
    }
}
