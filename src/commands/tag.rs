use crate::discord_client::CommandHandler;
use rusqlite::Connection;
use rusqlite::NO_PARAMS;

trait TagRepository {
    fn create_tag(&self, tag_name: &str, tag_body: &str) -> Result<(), Box<std::error::Error>>;
    fn read_tag(&self, tag_name: &str) -> Result<String, Box<std::error::Error>>;
    fn update_tag(&self, tag_name: &str, tag_body: &str) -> Result<(), Box<std::error::Error>>;
    fn delete_tag(&self, tag_name: &str) -> Result<(), Box<std::error::Error>>;
}

pub struct TagCommand {
    tag_repository: Box<TagRepository>,
}
pub struct SqliteTagRepository {
    db_connection: Connection,
}

struct Tag {
    name: String,
    body: String,
}

fn parse_ntag(command: &str) -> Option<Tag> {
    let mut result: Option<Tag> = None;

    let mut broken_down_command = command.split_whitespace().skip(1);
    let name = broken_down_command.next();
    let body: Vec<&str> = broken_down_command.collect();
    let body: String = body.join(" ");
    let body: Option<String> = if body.is_empty() { None } else { Some(body) };

    if let Some(name) = name {
        if let Some(body) = body {
            result = Some(Tag {
                name: String::from(name),
                body,
            });
        }
    }

    result
}

impl TagCommand {
    fn new() -> Result<TagCommand, Box<std::error::Error>> {
        let tag_repository = Box::new(SqliteTagRepository::new()?);

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
            match parse_ntag(command) {
                Some(new_tag) => {
                    self.tag_repository.create_tag(&new_tag.name, &new_tag.body);
                    send_message_callback(&format!(
                        "Created new tag {} with body {}",
                        &new_tag.name, &new_tag.body
                    ));
                }
                None => send_message_callback("Syntax error, could not create tag"),
            }
        } else if command.starts_with("atags") {

        } else {

        }

        Ok(())
    }
}

impl SqliteTagRepository {
    fn new() -> Result<SqliteTagRepository, Box<std::error::Error>> {
        let db_connection = Connection::open("tags.db")?;

        db_connection.execute(
            "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            body TEXT NOT NULL
        )",
            NO_PARAMS,
        )?;

        Ok(SqliteTagRepository { db_connection })
    }
}

impl TagRepository for SqliteTagRepository {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ntag() {
        let result = parse_ntag("ntag tag_name tag_body").unwrap();
        assert_eq!("tag_name", result.name);
        assert_eq!("tag_body", result.body);

        let result =
            parse_ntag("ntag tag_name long tag body with whitespace and 日本語").unwrap();
        assert_eq!("tag_name", result.name);
        assert_eq!("long tag body with whitespace and 日本語", result.body);
    }
}
