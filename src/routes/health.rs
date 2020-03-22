use actix_web::web::Json;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct HealthResponse {
    status: String
}

pub fn health() -> Result<Json<HealthResponse>, ()> {
    Ok(Json(HealthResponse {
        status: "OK".into()
    }))
}
