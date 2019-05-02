use crate::discord_client::CommandHandler;
use rusqlite::Connection;
use rusqlite::NO_PARAMS;

pub struct TagCommand {
    tag_repository: TagRepository,
}
pub struct TagRepository {
    db_connection: Connection,
}

impl TagCommand {
    fn new() -> Result<Self, Box<std::error::Error>> {
        let tag_repository = TagRepository::new()?;

        Ok(TagCommand { tag_repository })
    }
}

impl CommandHandler for TagCommand {
    fn process_command(
        &self,
        command: &str,
        send_message_callback: &Fn(&str) -> (),
    ) -> Result<(), Box<std::error::Error>> {
        if command.starts_with("ntag") {

        } else if command.starts_with("atags") {

        } else {

        }
    }
}

impl TagRepository {
    fn new() -> Result<TagRepository, Box<std::error::Error>> {
        let db_connection = Connection::open("tags.db")?;

        db_connection.execute(
            "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            body TEXT NOT NULL
        )",
            NO_PARAMS,
        )?;

        Ok(TagRepository { db_connection })
    }

    fn create_tag(&self, tag_name: &str, tag_body: &str) -> Result<(), Box<std::error::Error>> {
        self.db_connection.execute(
            "INSERT INTO tags
            (name, body) VALUES (?1, ?1)",
            &[tag_name, tag_body],
        )?;
        Ok(())
    }

    fn read_tag(&self, tag_name: &str) -> Result<String, Box<std::error::Error>> {
        Ok(self
            .db_connection
            .prepare(
                "SELECT body
                FROM tags 
                WHERE name = (?1)",
            )?
            .query_row(&[tag_name], |row| Ok(row.get(0)?))?)
    }

    fn update_tag(&self, tag_name: &str, tag_body: &str) -> Result<(), Box<std::error::Error>> {
        self.db_connection.execute(
            "UPDATE tags
                SET body = (?2)
                WHERE name = (?1)",
            &[tag_name, tag_body],
        )?;

        Ok(())
    }

    fn delete_tag(&self, tag_name: &str) -> Result<(), Box<std::error::Error>> {
        self.db_connection.execute(
            "DELETE FROM tags
                WHERE name = (?1)",
            &[tag_name],
        )?;

        Ok(())
    }
}
