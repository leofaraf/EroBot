use std::arch::asm;
use std::fs::File;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Read, Write};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub id: Uuid,
    pub name: String,
    pub photo_path: String,
    pub posts: Vec<Post>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub name: String,
    pub media: String,
    pub is_vip: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    pub path: String,
    pub media_type: MediaType
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MediaType {
    Image,
    Video
}

const MODELS_DB_PATH: &str = "./models.json";

pub async fn flush_models(models: Vec<Model>) {
    let mut connection =
        open_connection(MODELS_DB_PATH).await.unwrap();
    let str_json = serde_json::to_string(&models).unwrap();
    connection.write_all(str_json.as_bytes()).unwrap();
    connection.sync_all().unwrap();
}

pub async fn get_models() -> Vec<Model> {
    let text = get_models_as_str().await;
    let models: Vec<Model> = serde_json::from_str(text.as_str()).unwrap();
    models
}

pub async fn get_models_as_str() -> String {
    std::fs::read_to_string(MODELS_DB_PATH).unwrap()
}

async fn open_connection(file_path: &str) -> Option<File> {
    let is_exists_file: bool = Path::new(file_path).exists();
    match is_exists_file {
        false => {
            File::open(file_path).ok()
        },
        true => {
            File::create(file_path).ok()
        }
    }
}

