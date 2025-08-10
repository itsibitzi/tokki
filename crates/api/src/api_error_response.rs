use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    message: String,
    prefer: Option<String>,
}

impl ApiErrorResponse {
    pub fn new(message: String, prefer: Option<String>) -> Self {
        Self { message, prefer }
    }
}

impl std::fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
