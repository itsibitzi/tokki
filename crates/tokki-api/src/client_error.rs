use snafu::Snafu;

use crate::ApiErrorResponse;

/// `ApiError` type is used for
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ClientError {
    #[snafu(display("Failed to parse URL path from {base_url} joining {path:?}"))]
    UrlPathParse {
        base_url: String,
        path: &'static str,
        source: url::ParseError,
    },
    #[snafu(display("Failed to parse JSON from {base_url}"))]
    JsonParse {
        base_url: String,
        source: reqwest::Error,
    },
    #[snafu(display("Failed to send request to {base_url}: {source}"))]
    Reqwest {
        base_url: String,
        source: reqwest::Error,
    },
    #[snafu(display("Bad response from server {base_url}: {response}"))]
    BadResponse {
        base_url: String,
        response: ApiErrorResponse,
    },
}

impl ClientError {
    pub fn base_url(&self) -> &str {
        match self {
            ClientError::UrlPathParse { base_url, .. } => base_url,
            ClientError::JsonParse { base_url, .. } => base_url,
            ClientError::Reqwest { base_url, .. } => base_url,
            ClientError::BadResponse { base_url, .. } => base_url,
        }
    }
}
