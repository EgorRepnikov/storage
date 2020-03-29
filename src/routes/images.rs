use actix_web::{web::{Path, Payload}, Error, HttpResponse};
use actix_files::NamedFile;
use base64::decode;
use serde::Serialize;
use bytes::BytesMut;
use futures::StreamExt;
use std::io::Write;

use crate::config::CONFIG;
use crate::utils::create_resource_dir;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StoreRequest {
    pub data: String,
    pub name: String,
    pub original_type: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StoreResponse {
    pub name: String
}

pub async fn get(params: Path<(String, String)>) -> Result<NamedFile, Error> {
    Ok(NamedFile::open(resolve_path(&params.0, &params.1))?)
}

pub async fn store(resource: Path<String>, mut payload: Payload) -> Result<HttpResponse, Error> {
    create_resource_dir(resource.to_string());
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
    }
    let image = serde_json::from_slice::<StoreRequest>(&body)?;
    let data = decode(&image.data).unwrap_or(vec!());
    let path = resolve_path(&resource, &format!("{}.{}", &image.name, &image.original_type));
    let mut file = std::fs::File::create(&path)?;
    file.write_all(data.as_slice())?;
    let img = image::open(&path).unwrap();
    let result_name = format!("{}.jpg", &image.name);
    img.save(resolve_path(&resource, &result_name)).unwrap();
    std::fs::remove_file(&path)?;
    Ok(HttpResponse::Created().json(StoreResponse { name: result_name }))
}

pub async fn remove(params: Path<(String, String)>) -> Result<String, Error> {
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
