use actix_web::web;

mod middleware;
mod health;
mod images;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .route("/health", web::get().to(health::health))
        .service(
            web::scope("/images/{resource}")
                .route("/{image_name:.*}", web::get().to(images::get))
                .route("/{image_name:.*}", web::delete().to(images::remove))
                .service(
                    web::scope("")
                        .wrap(middleware::basic_auth::Auth)
                        .route("", web::post().to(images::store))
                )
        );
}
