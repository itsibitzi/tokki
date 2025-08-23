use crate::server_error::ServerError;
use axum::Json;
use tokki_api::healthcheck::{HealthcheckRequest, HealthcheckResponse};

pub async fn get_healthcheck(
    Json(_req): Json<HealthcheckRequest>,
) -> Result<Json<HealthcheckResponse>, ServerError> {
    Ok(Json(HealthcheckResponse::new("ok")))
}
