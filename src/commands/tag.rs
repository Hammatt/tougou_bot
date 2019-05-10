use crate::data_access::tag_repository::TagRepository;
use crate::discord_client::CommandHandler;
use crate::models::tag::Tag;
use std::sync::Mutex;

pub struct TagCommand {
    tag_repository: Mutex<Box<TagRepository + Send>>,
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
    pub fn new(
        tag_repository: Box<TagRepository + Send>,
    ) -> Result<TagCommand, Box<std::error::Error>> {
        Ok(TagCommand {
            tag_repository: Mutex::new(tag_repository),
        })
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
                    match self.tag_repository.lock().unwrap().create_tag(
                        &new_tag.name,
                        &new_tag.body,
                        tennant_id,
                    ) {
                        Ok(()) => {
                            send_message_callback(&format!(
                                "新しいタッグ「{}」➡「{}」を作った。",
                                &new_tag.name, &new_tag.body
                            ));
                        }
                        Err(error) => {
                            send_message_callback(
                                "エラーが発生しました。そのタッグもう知っている。",
                            );
                            return Err(error);
                        }
                    }
                }
                None => send_message_callback("シンタックスエラーが発生しました"),
            }
        } else if command.starts_with("!atags") {
            let tags: Vec<Tag> = match self
                .tag_repository
                .lock()
                .unwrap()
                .read_all_tags(tennant_id)
            {
                Ok(tags) => tags,
                Err(error) => {
                    send_message_callback("タッグが見つかりません。");
                    return Err(error);
                }
            };

            if tags.is_empty() {
                send_message_callback("タッグがまだいない。");
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
                Some(tag_name) => match self
                    .tag_repository
                    .lock()
                    .unwrap()
                    .read_tag(tag_name, tennant_id)
                {
                    Ok(body) => {
                        send_message_callback(&body);
                    }
                    Err(error) => {
                        send_message_callback("そのタッグがいない");
                        return Err(error);
                    }
                },
                None => send_message_callback("シンタックスエラーが発生しました"),
            }
        }

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
