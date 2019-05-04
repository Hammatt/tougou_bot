use crate::data_access::tag_repository::TagRepository;
use crate::models::tag::Tag;
use rusqlite::Connection;
use rusqlite::NO_PARAMS;

pub struct SqliteTagRepository {
    db_connection: Connection,
}

impl SqliteTagRepository {
    pub fn new() -> Result<SqliteTagRepository, Box<std::error::Error>> {
        let db_connection = Connection::open("tags.db")?;

        db_connection.execute(
            "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            body TEXT NOT NULL,
            tennant_id INTEGER NOT NULL,
            UNIQUE (name, tennant_id)
        )",
            NO_PARAMS,
        )?;

        Ok(SqliteTagRepository { db_connection })
    }
}

impl TagRepository for SqliteTagRepository {
    fn create_tag(
        &self,
        tag_name: &str,
        tag_body: &str,
        tennant_id: u64,
    ) -> Result<(), Box<std::error::Error>> {
        //sqlite3 doesn't support u64 properly so we have to cast to i64 first. as long as we do this consistantly we shouldn't have problems.
        let tennant_id = (tennant_id as i64).to_string();

        self.db_connection.execute(
            "INSERT INTO tags
            (name, body, tennant_id) VALUES (?1, ?2, ?3)",
            &[tag_name, tag_body, &tennant_id],
        )?;
        Ok(())
    }

    fn read_tag(&self, tag_name: &str, tennant_id: u64) -> Result<String, Box<std::error::Error>> {
        //sqlite3 doesn't support u64 properly so we have to cast to i64 first. as long as we do this consistantly we shouldn't have problems.
        let tennant_id = (tennant_id as i64).to_string();

        Ok(self
            .db_connection
            .prepare(
                "SELECT body
                FROM tags 
                WHERE tennant_id = (?1) AND name = (?2)",
            )?
            .query_row(&[&tennant_id, tag_name], |row| Ok(row.get(0)?))?)
    }

    fn read_all_tags(&self, tennant_id: u64) -> Result<Vec<Tag>, Box<std::error::Error>> {
        //sqlite3 doesn't support u64 properly so we have to cast to i64 first. as long as we do this consistantly we shouldn't have problems.
        let tennant_id = (tennant_id as i64).to_string();

        Ok(self
            .db_connection
            .prepare(
                "SELECT name, body
                FROM tags
                WHERE tennant_id = (?1)",
            )?
            .query_map(&[&tennant_id], |row| {
                Ok(Tag {
                    name: row.get(0)?,
                    body: row.get(1)?,
                })
            })?
            .filter_map(Result::ok)
            .collect())
    }
}
