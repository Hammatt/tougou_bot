use crate::discord_client::CommandHandler;
use rusqlite::Connection;
use rusqlite::NO_PARAMS;

trait TagRepository {
    fn create_tag(
        &self,
        tag_name: &str,
        tag_body: &str,
        tennant_id: u64,
    ) -> Result<(), Box<std::error::Error>>;
    fn read_tag(&self, tag_name: &str, tennant_id: u64) -> Result<String, Box<std::error::Error>>;
    fn read_all_tags(&self, tennant_id: u64) -> Result<Vec<Tag>, Box<std::error::Error>>;
}

pub struct TagCommand {
    tag_repository: Box<TagRepository + Send>,
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

fn parse_tag_command(command: &str) -> Option<&str> {
    command.split_whitespace().nth(1)
}

impl TagCommand {
    pub fn new() -> Result<TagCommand, Box<std::error::Error>> {
        let tag_repository = Box::new(SqliteTagRepository::new()?);

        Ok(TagCommand { tag_repository })
    }
}

impl CommandHandler for TagCommand {
    fn process_command(
        &self,
        command: &str,
        tennant_id: u64,
        send_message_callback: &Fn(&str) -> (),
    ) -> Result<(), Box<std::error::Error>> {
        if command.starts_with("!ntag") {
            match parse_ntag(command) {
                Some(new_tag) => {
                    self.tag_repository
                        .create_tag(&new_tag.name, &new_tag.body, tennant_id)?;
                    send_message_callback(&format!(
                        "Created new tag {} with body {}",
                        &new_tag.name, &new_tag.body
                    ));
                }
                None => send_message_callback("Syntax error, could not create tag"),
            }
        } else if command.starts_with("!atags") {
            let tags: Vec<Tag> = self.tag_repository.read_all_tags(tennant_id)?;
            if tags.is_empty() {
                send_message_callback("no tags created");
            } else {
                let mut message = String::new();

                for tag in tags {
                    message.push_str(&tag.name);
                    message.push_str(": ");
                    message.push_str(&tag.body);
                    message.push('\n')
                }

                send_message_callback(&message);
            }
        } else {
            match parse_tag_command(command) {
                Some(tag_name) => {
                    let body = self.tag_repository.read_tag(tag_name, tennant_id)?;

                    send_message_callback(&body);
                }
                None => send_message_callback("Syntax error, could not find tag"),
            }
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
