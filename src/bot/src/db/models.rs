use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub tg_id: String,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserRole {
    Free,
    Vip
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub id: Uuid,
    pub name: String,
    pub category: ModelCategory,
    pub media: Media,
    pub posts: Vec<Post>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ModelCategory {
    Influential,
    Tiktok,
    Star,
    Twitch,
    OnlyFans,
    Boosty
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: Uuid,
    pub name: String,
    pub media: Media,
    pub is_vip: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Media {
    pub path: String,
    pub media_type: MediaType
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MediaType {
    Image,
    Video
}

pub const MODELS_DB_PATH: &str = "./models.json";
pub const USER_DB_PATH: &str = "./user.json";