use crate::models::jisho_definition::JishoDefinition;

pub trait JishoRepository {
    fn get_definition(&self, query: Vec<&str>) -> Result<JishoDefinition, Box<std::error::Error>>;
}

pub mod jisho_org_repository;
