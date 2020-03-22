use actix_web::web::Path;
use actix_files::NamedFile;


use super::super::config::CONFIG;

pub async fn get(params: Path<(String, String)>) -> Result<NamedFile, std::io::Error> {
    let path = std::path::Path::new(&std::env::current_dir().unwrap())
        .join(&CONFIG.storage_dir)
        .join(&CONFIG.images_storage_dir)
        .join(params.0.to_string())
        .join(params.1.to_string());
    Ok(NamedFile::open(path)?)
}
