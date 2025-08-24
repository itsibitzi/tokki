use std::{
    io,
    sync::{Arc, Mutex},
};
use tokki_api::TokkiClient;
use tokki_common::{Offset, Record};

use crate::{replication::Replication, storage::StorageEngine};

#[derive(Clone)]
pub enum AppState {
    Leader {
        token: String,
        storage: StorageEngine,
        replication: Arc<Mutex<Replication>>,
        required_replicas: usize,
    },
    Follower {
        storage: StorageEngine,
        leader_client: TokkiClient,
    },
}

impl AppState {
    /// Get the current maximum offset
    #[allow(dead_code)]
    pub async fn max_offset(&self) -> io::Result<Option<Offset>> {
        match self {
            AppState::Leader { storage, .. } => storage.max_offset().await,
            AppState::Follower { storage, .. } => storage.max_offset().await,
        }
    }

    /// Put a record on the log, returning it's offset.
    #[allow(dead_code)]
    pub async fn put_record(&self, record: Record) -> io::Result<Offset> {
        match self {
            AppState::Leader { storage, .. } => storage.put_record(record).await,
            AppState::Follower { storage, .. } => storage.put_record(record).await,
        }
    }

    /// Get `max_records` number off `Records` from the provided `offset`.
    pub async fn get_records(
        &self,
        offset: Offset,
        max_records: usize,
    ) -> io::Result<(Vec<Record>, Offset)> {
        match self {
            AppState::Leader { storage, .. } => storage.get_records(offset, max_records).await,
            AppState::Follower { storage, .. } => storage.get_records(offset, max_records).await,
        }
    }
}
