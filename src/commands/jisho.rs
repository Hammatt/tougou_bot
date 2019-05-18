use crate::data_access::jisho_repository::JishoRepository;
use crate::discord_client::CommandHandler;

pub struct JishoCommand {
    jisho_repository: Box<JishoRepository + Send>,
}

fn parse_command(command: &str) -> Vec<&str> {
    command.split_ascii_whitespace().skip(1).collect()
}

impl JishoCommand {
    pub fn new(jisho_repository: Box<JishoRepository + Send>) -> JishoCommand {
        JishoCommand { jisho_repository }
    }
}

impl CommandHandler for JishoCommand {
    fn process_command(
        &self,
        command: &str,
        _tennant_id: u64,
        send_message_callback: &Fn(&str) -> (),
    ) -> Result<(), Box<std::error::Error>> {
        let query = parse_command(command);

        match self.jisho_repository.get_definition(query) {
            Ok(definition) => {
                send_message_callback(&format!(
                    "言葉：{}\n読み方：{}\n言葉の意味：{}\n続き：<{}>",
                    definition.word,
                    definition.reading,
                    definition.english_definitions.join("; "),
                    definition.link_for_more,
                ));
                Ok(())
            }
            Err(error) => {
                send_message_callback("言葉を取得できない");
                Err(error)
            }
        }
    }
}
