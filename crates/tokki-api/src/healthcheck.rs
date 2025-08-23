use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthcheckRequest {}

impl HealthcheckRequest {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthcheckResponse {
    status: String,
}

impl HealthcheckResponse {
    pub fn new(status: impl Into<String>) -> Self {
        Self {
            status: status.into(),
        }
    }
}
