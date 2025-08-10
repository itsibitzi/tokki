use common::{Offset, Record};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PutRecordsRequest {
    pub records: Vec<Record>,
}

impl PutRecordsRequest {
    pub fn single(record: Record) -> Self {
        Self {
            records: vec![record],
        }
    }

    pub fn new(records: Vec<Record>) -> Self {
        Self { records }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutRecordsResponse {
    pub offset: Offset,
    pub len: usize,
}

impl PutRecordsResponse {
    pub fn new(offset: Offset, len: usize) -> Self {
        Self { offset, len }
    }
}
