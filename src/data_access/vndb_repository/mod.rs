use crate::models::vndb_result::VNDBResult;

pub trait VNDBRepository {
    fn get_visual_novel(&self, query: Vec<&str>) -> Result<VNDBResult, Box<std::error::Error>>;
}

pub mod vndb_org_repository;
