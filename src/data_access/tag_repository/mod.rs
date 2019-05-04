use crate::models::tag::Tag;

pub trait TagRepository {
    fn create_tag(
        &self,
        tag_name: &str,
        tag_body: &str,
        tennant_id: u64,
    ) -> Result<(), Box<std::error::Error>>;
    fn read_tag(&self, tag_name: &str, tennant_id: u64) -> Result<String, Box<std::error::Error>>;
    fn read_all_tags(&self, tennant_id: u64) -> Result<Vec<Tag>, Box<std::error::Error>>;
}

pub mod sqlite_tag_repository;
