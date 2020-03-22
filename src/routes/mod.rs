use actix_web::web;

mod health;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .route("/health", web::get().to(health::health));
}
