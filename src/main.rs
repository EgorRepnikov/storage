#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use actix_web::{App, HttpServer};

use crate::config::CONFIG;
use crate::routes::routes;
use crate::utils::create_storage_dirs;

mod config;
mod routes;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    create_storage_dirs();
    HttpServer::new(|| {
        App::new()
            .configure(routes)
    })
        .bind(&CONFIG.server)?
        .run().await
}
