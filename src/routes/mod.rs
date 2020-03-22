use actix_web::web;

mod health;

use crate::health::health;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .route("/health", web::get().to(health));
}
