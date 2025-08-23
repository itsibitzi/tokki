use serde::{Deserialize, Serialize};
use tokki_common::{
    Offset, Record,
    hmac::{HmacSha256, HmacValue},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplicateLogRequest {
    pub follower_url: String,
    pub max_acknowledged_offset: Option<Offset>,
}

impl ReplicateLogRequest {
    pub fn new(follower_url: String, max_acknowledged_offset: Option<Offset>) -> Self {
        Self {
            follower_url,
            max_acknowledged_offset,
        }
    }
}

impl HmacValue for ReplicateLogRequest {
    fn update_mac(&self, mac: &mut HmacSha256) {
        self.follower_url.update_mac(mac);
        self.max_acknowledged_offset.update_mac(mac);
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReplicateLogResponse {
    pub records: Vec<Record>,
}

impl ReplicateLogResponse {
    pub fn new(records: Vec<Record>) -> Self {
        Self { records }
    }
}

impl HmacValue for ReplicateLogResponse {
    fn update_mac(&self, mac: &mut HmacSha256) {
        for record in &self.records {
            record.update_mac(mac);
        }
    }
}
