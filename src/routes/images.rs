use actix_web::web::{Path, Json};
use actix_files::NamedFile;
use base64::decode;
use serde::Serialize;

use super::super::config::CONFIG;
use std::io::Write;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StoreRequest {
    pub data: String,
    pub name: String
}

pub async fn get(params: Path<(String, String)>) -> Result<NamedFile, std::io::Error> {
    Ok(NamedFile::open(resolve_path(&params.0, &params.1))?)
}

pub async fn store(resource: Path<String>, image: Json<StoreRequest>) -> Result<String, std::io::Error> {
    let data = decode(&image.data).unwrap_or(vec!());
    let mut file = std::fs::File::create(resolve_path(&resource, &image.name))?;
    file.write_all(data.as_slice())?;
    Ok("Created".to_string())
}

pub async fn remove(params: Path<(String, String)>) -> Result<String, std::io::Error> {
    std::fs::remove_file(resolve_path(&params.0, &params.1))?;
    Ok("Deleted".to_string())
}

fn resolve_path(resource: &String, name: &String) -> std::path::PathBuf {
    std::path::Path::new(&std::env::current_dir().unwrap())
        .join(&CONFIG.storage_dir)
        .join(&CONFIG.images_storage_dir)
        .join(resource.to_string())
        .join(name.to_string())
}
