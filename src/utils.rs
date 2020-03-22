use std::path::Path;
use std::fs::create_dir;

use crate::config::CONFIG;

pub fn create_storage_dirs() {
    let storage_path = Path::new(&CONFIG.storage_dir);
    let _ = create_dir(storage_path);
    let _ =create_dir(storage_path.join(&CONFIG.images_storage_dir));
}

pub fn create_resource_dir(resource: String) {
    let resource_storage_path = Path::new(&CONFIG.storage_dir)
        .join(&CONFIG.images_storage_dir)
        .join(resource.as_str());
    let _ = create_dir(resource_storage_path);
}
