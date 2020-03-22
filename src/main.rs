use actix_web::{App, HttpServer};

use crate::config::CONFIG;
use crate::routes::routes;

mod config;
mod routes;

#[actix_rt:main]
fn main() -> std::io::Result<()> {
    dotenv::dotenv.ok();
    HttpServer::new(|| {
        App::new()
            .configure(routes)
    })
        .bind(&CONFIG.server)?
        .run().await
}
