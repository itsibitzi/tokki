use crate::controller_error::ControllerError;
use axum::Json;
use tokki_api::healthcheck::{HealthcheckRequest, HealthcheckResponse};

pub async fn get_healthcheck(
    Json(_req): Json<HealthcheckRequest>,
) -> Result<Json<HealthcheckResponse>, ControllerError> {
    Ok(Json(HealthcheckResponse::new("ok")))
}
