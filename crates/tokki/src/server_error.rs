use std::io;

use axum::{Json, body::Body, http::Response, response::IntoResponse};
use reqwest::StatusCode;
use snafu::Snafu;
use tokki_api::{ApiErrorResponse, ClientError};
use tokki_common::hmac::HmacError;

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
    #[snafu(display("I/O error"))]
    Io { source: io::Error },
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
            ServerError::Io { .. } => (StatusCode::INTERNAL_SERVER_ERROR, None),
        };

        let body = Json(ApiErrorResponse::new(message, prefer));

        (status, body).into_response()
    }
}
