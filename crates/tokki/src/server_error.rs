use api::{ApiErrorResponse, ClientError};
use axum::{Json, body::Body, http::Response, response::IntoResponse};
use common::hmac::HmacError;
use reqwest::StatusCode;
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ServerError {
    #[snafu(display("Failed to replicate after {timeout_s}s"))]
    Replication { timeout_s: i32 },
    #[snafu(display("HMAC signature invalid"))]
    Hmac { source: HmacError },
    #[snafu(display("Failure when forwarding to leader"))]
    LeaderForwarding { source: ClientError, leader: String },
    #[snafu(display("Follower cannot service this request"))]
    IsFollower { leader: String },
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response<Body> {
        let message = self.to_string();
        tracing::error!("Error servicing request: {:?}", self);

        let (status, prefer) = match self {
            ServerError::Replication { .. } => (StatusCode::INTERNAL_SERVER_ERROR, None),
            ServerError::Hmac { .. } => (StatusCode::UNAUTHORIZED, None),
            ServerError::LeaderForwarding { leader, .. } => {
                (StatusCode::INTERNAL_SERVER_ERROR, Some(leader))
            }
            ServerError::IsFollower { leader } => (StatusCode::MISDIRECTED_REQUEST, Some(leader)),
        };

        let body = Json(ApiErrorResponse::new(message, prefer));

        (status, body).into_response()
    }
}
