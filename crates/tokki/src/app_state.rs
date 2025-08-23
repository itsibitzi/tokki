use std::sync::{Arc, Mutex};
use tokki_api::TokkiClient;

use crate::{replication::Replication, storage::Storage};

#[derive(Clone)]
pub enum AppState<S: Storage> {
    Leader {
        token: String,
        storage: S,
        replication: Arc<Mutex<Replication>>,
        required_replicas: usize,
    },
    Follower {
        storage: S,
        leader_client: TokkiClient,
    },
}

impl<S: Storage> AppState<S> {
    pub fn storage(&self) -> &S {
        match self {
            AppState::Leader { storage, .. } => storage,
            AppState::Follower { storage, .. } => storage,
        }
    }
}
