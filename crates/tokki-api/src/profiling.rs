use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FinishProfilingResponse {
    pub flamegraph: String,
}
