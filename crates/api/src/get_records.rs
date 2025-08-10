use common::{Offset, Record};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetRecordsRequest {
    pub offset: Offset,
    pub max_records: usize,
}

impl GetRecordsRequest {
    pub fn new(offset: Offset, max_records: usize) -> Self {
        Self {
            offset,
            max_records,
        }
    }

    pub fn offset(&self) -> Offset {
        self.offset
    }

    pub fn max_records(&self) -> usize {
        self.max_records
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRecordsResponse {
    records: Vec<Record>,
    next_offset: Offset,
}

impl GetRecordsResponse {
    pub fn new(records: Vec<Record>, next_offset: Offset) -> Self {
        Self {
            records,
            next_offset,
        }
    }

    pub fn records(&self) -> &[Record] {
        &self.records
    }

    pub fn next_offset(&self) -> Offset {
        self.next_offset
    }
}
