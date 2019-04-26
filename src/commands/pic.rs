use crate::discord_client::CommandHandler;
use reqwest;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct DanbooruApiResponse {
    file_url: String,
}

pub struct PicCommand;

impl CommandHandler for PicCommand {
    fn process_command(
        &self,
        command: &str,
        send_message_callback: &Fn(&str) -> (),
    ) -> Result<(), Box<std::error::Error>> {
        let parameters = get_search_parameters(command);
        let parameters = format_search_parameters(parameters);

        let request_uri: String = format!(
            "https://danbooru.donmai.us/posts.json?search&random=true&limit=1&tags={}",
            parameters
        );
        let response_body: String = reqwest::get(&request_uri)?.text()?;

        let response_model: Vec<DanbooruApiResponse> =
            serde_json::from_str(&response_body).unwrap();

        let result = match response_model.first() {
            Some(picture) => &picture.file_url,
            None => "No images with those tags found",
        };

        send_message_callback(result);

        Ok(())
    }
}

fn format_search_parameters(parameters: Vec<&str>) -> String {
    parameters.join("+")
}

fn get_search_parameters(command: &str) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();

    let paramaters: Vec<&str> = command.split_whitespace().collect();
    for word in paramaters.iter().skip(1) {
        result.push(word);
    }

    result
}
