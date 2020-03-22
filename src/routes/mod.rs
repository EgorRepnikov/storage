use actix_web::web;

mod health;
mod images;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .route("/health", web::get().to(health::health))
        .service(
            web::scope("/images/{resource}")
                .route("/{image_name:.*}", web::get().to(images::get))
                .route("/", web::post().to(images::store))
        );
}
