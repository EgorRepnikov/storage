use actix_web::web::Json;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct HealthResponse {
    status: String
}

pub async fn health() -> Result<Json<HealthResponse>, ()> {
    Ok(Json(HealthResponse {
        status: "OK".into()
    }))
}
