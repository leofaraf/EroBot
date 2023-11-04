pub mod models;

use std::arch::asm;
use std::error::Error;
use std::fs::File;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Read, Write};
use std::path::Path;
use uuid::Uuid;
use self::models::*;

pub async fn flush_users(models: Vec<User>) {
    let mut connection =
        open_connection(USER_DB_PATH).await.unwrap();
    let str_json = serde_json::to_string(&models).unwrap();
    connection.write_all(str_json.as_bytes()).unwrap();
    connection.flush().unwrap();
}

pub async fn get_users() -> Vec<User> {
    let mut text = get_users_as_str().await;
    if let Some(text) = text {
        let models = serde_json::from_str(text.as_str());
        if let Ok(models) = models {
            return models
        }
    }
    vec![]
}

pub async fn get_users_as_str() -> Option<String> {
    std::fs::read_to_string(USER_DB_PATH).ok()
}

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

// async fn remove_file(file_path: &str) {
//     let is_exists_file: bool = Path::new(file_path).exists();
//     if is_exists_file {
//         File::
//     }
// }

async fn open_connection(file_path: &str) -> Option<File> {
    let is_exists_file: bool = Path::new(file_path).exists();
    match is_exists_file {
        true => {
            println!("opening file: {}", file_path);
            File::create(file_path).ok()
        },
        false => {
            println!("creating file: {}", file_path);
            File::create(file_path).ok()
        }
    }
}