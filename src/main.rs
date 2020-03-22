use actix_web::{App, HttpServer};

use crate::config::CONFIG;

mod config;

#[actix_rt:main]
fn main() -> std::io::Result<()> {
    dotenv::dotenv.ok();
    HttpServer::new(|| {
        App::new()
    })
        .bind(&CONFIG.server)?
        .run().await
}
