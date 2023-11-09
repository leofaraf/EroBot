use std::ops::Add;
use std::path::Path;
use teloxide::Bot;
use teloxide::net::Download;
use teloxide::prelude::Requester;
use teloxide::types::{Document, Video};
use tokio::fs::{create_dir_all, File};
use uuid::Uuid;
use crate::db::models::{Media, MediaType};

pub async fn add_document(document: &Document, bot: &Bot, media_type: MediaType) -> Result<Media, ()> {
    let name = document.file_name.clone().unwrap();

    let extension = match Path::new(name.as_str()).extension() {
        None => None,
        Some(path) => path.to_str()
    };

    let folder = "media/";
    let path: &Path = Path::new(folder);
    if !path.exists() {
        create_dir_all(path).await.expect("Can't create dirs");
    }

    let media = Media {
        media_type,
        path: folder.to_string().add(match extension {
            Some(text) => Uuid::new_v4().to_string().add(".").add(text),
            None => Uuid::new_v4().to_string()
        }.as_str())
    };

    let file =  bot.get_file(&document.file.id).await.unwrap();
    let mut dst = File::create(&media.path).await.unwrap();

    // Download the voice message and write it to the file
    bot.download_file(&file.path, &mut dst).await.unwrap();
    Ok((media))
}

pub async fn add_video(document: &Video, bot: &Bot, media_type: MediaType) -> Result<Media, ()> {
    let name = document.file_name.clone().unwrap();

    let extension = match Path::new(name.as_str()).extension() {
        None => None,
        Some(path) => path.to_str()
    };

    let folder = "media/";
    let path: &Path = Path::new(folder);
    if !path.exists() {
        create_dir_all(path).await.expect("Can't create dirs");
    }

    let media = Media {
        media_type,
        path: folder.to_string().add(match extension {
            Some(text) => Uuid::new_v4().to_string().add(".").add(text),
            None => Uuid::new_v4().to_string()
        }.as_str())
    };

    let file =  bot.get_file(&document.file.id).await.unwrap();
    let mut dst = File::create(&media.path).await.unwrap();

    // Download the voice message and write it to the file
    bot.download_file(&file.path, &mut dst).await.unwrap();
    Ok((media))
}