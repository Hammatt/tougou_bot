use crate::data_access::jisho_repository::JishoRepository;
use crate::models::jisho_definition::JishoDefinition;
use reqwest;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct JishoOrgApiResponse {
    data: Vec<Data>,
}

#[derive(Deserialize)]
struct Data {
    japanese: Vec<Japanese>,
    senses: Vec<Senses>,
}

#[derive(Deserialize)]
struct Japanese {
    word: Option<String>,
    reading: String,
}

#[derive(Deserialize)]
struct Senses {
    english_definitions: Vec<String>,
    parts_of_speech: Vec<String>,
}

pub struct JishoOrgRepository;

#[derive(Debug)]
struct JishoOrgRepositoryError {
    description: String,
}

impl JishoOrgRepository {
    pub fn default() -> JishoOrgRepository {
        JishoOrgRepository
    }
}

fn format_search_parameters(parameters: Vec<&str>) -> String {
    parameters.join("%20")
}

impl JishoRepository for JishoOrgRepository {
    fn get_definition(&self, query: Vec<&str>) -> Result<JishoDefinition, Box<std::error::Error>> {
        let parameters = format_search_parameters(query);
        let request_uri = format!(
            "https://jisho.org/api/v1/search/words?keyword={}",
            parameters
        );

        let response_body: String = reqwest::get(&request_uri)?.text()?;

        let response_models: JishoOrgApiResponse = serde_json::from_str(&response_body)?;

        let first_response_model_data = response_models.data.first().ok_or_else(|| {
            JishoOrgRepositoryError::new(String::from("Got no item in response from jisho api"))
        })?;

        let japanese_section = first_response_model_data.japanese.first().ok_or_else(|| {
            JishoOrgRepositoryError::new(String::from("japanese section was empty"))
        })?;

        let senses_section = first_response_model_data.senses.first().ok_or_else(|| {
            JishoOrgRepositoryError::new(String::from("senses section was empty"))
        })?;

        Ok(JishoDefinition {
            word: match japanese_section
                .word.clone() {
                    Some(word) => word,
                    None => japanese_section.reading.clone(),
                },
            reading: japanese_section.reading.clone(),
            english_definitions: senses_section.english_definitions.clone(),
            parts_of_speech: senses_section.parts_of_speech.clone(),
            link_for_more: format!("https://jisho.org/search/{}", parameters),
        })
    }
}

impl JishoOrgRepositoryError {
    fn new(description: String) -> JishoOrgRepositoryError {
        JishoOrgRepositoryError { description }
    }
}

impl std::error::Error for JishoOrgRepositoryError {}

impl std::fmt::Display for JishoOrgRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}
