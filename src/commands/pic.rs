use crate::data_access::pic_repository::PicRepository;
use crate::discord_client::CommandHandler;

pub struct PicCommand {
    pic_repository: Box<PicRepository + Send>,
}

impl PicCommand {
    pub fn new(pic_repository: Box<PicRepository + Send>) -> PicCommand {
        PicCommand { pic_repository }
    }
}

impl CommandHandler for PicCommand {
    fn process_command(
        &self,
        command: &str,
        _tennant_id: u64,
        send_message_callback: &Fn(&str) -> (),
    ) -> Result<(), Box<std::error::Error>> {
        let parameters = get_search_parameters(command);

        match self.pic_repository.get_random_picture(&parameters) {
            Ok(result) => {
                send_message_callback(&result.uri);
                Ok(())
            }
            Err(error) => {
                send_message_callback(
                    "Failed to get image. Check you have at most two query parameters.",
                );
                Err(error)
            }
        }
    }
}

fn get_search_parameters(command: &str) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();

    let paramaters: Vec<&str> = command.split_whitespace().collect();
    for word in paramaters.iter().skip(1) {
        result.push(word);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_search_parameters() {
        let empty_vec: Vec<&str> = Vec::new();
        assert_eq!(empty_vec, get_search_parameters("pic"));
        assert_eq!(
            vec!["tag1", "tag2", "tag3"],
            get_search_parameters("pic tag1 tag2 tag3")
        );
        assert_eq!(
            vec!["一番", "に", "サン"],
            get_search_parameters("pic 一番 に サン")
        );
    }
}
