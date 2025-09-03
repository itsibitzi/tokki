use std::{io, string::FromUtf8Error};

use axum::{Json, body::Body, http::Response, response::IntoResponse};
use reqwest::StatusCode;
use snafu::Snafu;
use tokki_api::{ApiErrorResponse, ClientError};
use tokki_common::hmac::HmacError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ControllerError {
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
    // Profiling
    #[snafu(display("Profiling is disabled"))]
    ProfilingDisabled,
    #[snafu(display("Profiler is not running"))]
    ProfilingNotActive,
    #[snafu(display("Profiler is already running"))]
    ProfilingActive,
    #[snafu(display("Profiler mutex poisoned"))]
    ProfilingMutexPoisoned,
    #[snafu(display("Failed to start profiler"))]
    ProfilingStartFailed { source: pprof::Error },
    #[snafu(display("Failed to create profiling report"))]
    ProfilingReport { source: pprof::Error },
    #[snafu(display("Failed to create flamegraph"))]
    Flamegraph { source: FromUtf8Error },
}

impl IntoResponse for ControllerError {
    fn into_response(self) -> Response<Body> {
        let message = self.to_string();
        tracing::error!("Error servicing request: {:?}", self);

        let (status, prefer) = match self {
            ControllerError::Replication { .. } => (StatusCode::INTERNAL_SERVER_ERROR, None),
            ControllerError::Hmac { .. } => (StatusCode::UNAUTHORIZED, None),
            ControllerError::LeaderForwarding { leader, .. } => {
                (StatusCode::INTERNAL_SERVER_ERROR, Some(leader))
            }
            ControllerError::IsFollower { leader } => {
                (StatusCode::MISDIRECTED_REQUEST, Some(leader))
            }
            ControllerError::Io { .. } => (StatusCode::INTERNAL_SERVER_ERROR, None),
            ControllerError::ProfilingDisabled => (StatusCode::FORBIDDEN, None),
            ControllerError::ProfilingNotActive => (StatusCode::BAD_REQUEST, None),
            ControllerError::ProfilingActive => (StatusCode::BAD_REQUEST, None),
            ControllerError::ProfilingMutexPoisoned => (StatusCode::INTERNAL_SERVER_ERROR, None),
            ControllerError::ProfilingStartFailed { .. } => {
                (StatusCode::INTERNAL_SERVER_ERROR, None)
            }
            ControllerError::ProfilingReport { .. } => (StatusCode::INTERNAL_SERVER_ERROR, None),
            ControllerError::Flamegraph { .. } => (StatusCode::INTERNAL_SERVER_ERROR, None),
        };

        let body = Json(ApiErrorResponse::new(message, prefer));

        (status, body).into_response()
    }
}
