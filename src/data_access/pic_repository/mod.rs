use crate::models::pic::Pic;

pub trait PicRepository {
    fn get_random_picture(&self, query: &[&str]) -> Result<Pic, Box<std::error::Error>>;
}

pub mod danbooru_pic_repository;
