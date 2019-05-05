use crate::data_access::pic_repository::PicRepository;
use crate::models::pic::Pic;
use reqwest;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct DanbooruApiResponse {
    file_url: String,
}

pub struct DanbooruPicRepository;

#[derive(Debug)]
pub struct DanbooruPicRepositoryError {
    description: String,
}

fn format_search_parameters(parameters: &[&str]) -> String {
    parameters.join("+")
}

impl DanbooruPicRepository {
    pub fn default() -> DanbooruPicRepository {
        DanbooruPicRepository
    }
}

impl PicRepository for DanbooruPicRepository {
    fn get_random_picture(&self, query: &[&str]) -> Result<Pic, Box<std::error::Error>> {
        if query.len() > 2 {
            Err(Box::new(DanbooruPicRepositoryError::new(
                "Too many args. danbooru supports up to 2 query parameters to be provided"
                    .to_string(),
            )))
        } else {
            let parameters = format_search_parameters(query);
            let request_uri: String = format!(
                "https://danbooru.donmai.us/posts.json?search&random=true&limit=1&tags={}",
                parameters
            );
            let response_body: String = reqwest::get(&request_uri)?.text()?;

            let response_model: Vec<DanbooruApiResponse> = serde_json::from_str(&response_body)?;

            let uri = match response_model.first() {
                Some(picture) => Ok(picture.file_url.clone()),
                None => Err(DanbooruPicRepositoryError::new(
                    "No images with those tags found".to_string(),
                )),
            }?;

            Ok(Pic { uri })
        }
    }
}

impl DanbooruPicRepositoryError {
    fn new(description: String) -> DanbooruPicRepositoryError {
        DanbooruPicRepositoryError { description }
    }
}

impl std::error::Error for DanbooruPicRepositoryError {}

impl std::fmt::Display for DanbooruPicRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_search_parameters() {
        let empty_vec: Vec<&str> = Vec::new();
        assert_eq!("", format_search_parameters(&empty_vec));
        assert_eq!(
            "tag1+tag2+tag3",
            format_search_parameters(&vec!["tag1", "tag2", "tag3"])
        );
        assert_eq!(
            "一番+に+サン",
            format_search_parameters(&vec!["一番", "に", "サン"])
        );
    }
}
