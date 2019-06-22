use crate::data_access::vndb_repository::VNDBRepository;
use crate::discord_client::CommandHandler;
use crate::models::vndb_result::VNDBResult;

pub struct VNDBCommand {
    vndb_repository: Box<VNDBRepository + Send>,
}

fn parse_command(command: &str) -> Vec<&str> {
    command.split_ascii_whitespace().skip(1).collect()
}

impl VNDBCommand {
    pub fn new(vndb_repository: Box<VNDBRepository + Send>) -> VNDBCommand {
        VNDBCommand { vndb_repository }
    }
}

impl CommandHandler for VNDBCommand {
    fn process_command(
        &self,
        command: &str,
        _tennant_id: u64,
        send_message_callback: &Fn(&str) -> (),
    ) -> Result<(), Box<std::error::Error>> {
        let query = parse_command(command);

        match self.vndb_repository.get_visual_novel(query) {
            Ok(result) => {
                match result {
                    VNDBResult::Single(uri) => {
                        send_message_callback(&uri);
                    }
                    VNDBResult::MostLikelyAndMore(suggested_uri, more_results) => {
                        send_message_callback(&format!(
                            "{}\n続き：<{}>",
                            suggested_uri, more_results
                        ));
                    }
                    VNDBResult::None => {
                        send_message_callback("エラーが発生しました"); //TODO: improve error message
                    }
                }

                Ok(())
            }
            Err(error) => {
                send_message_callback("エラーが発生しました"); //TODO: improve error message
                Err(error)
            }
        }
    }
}
