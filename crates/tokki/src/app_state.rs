use api::TokkiClient;
use std::sync::{Arc, Mutex};

use crate::{log::Log, replication::Replication};

#[derive(Clone)]
pub enum AppState {
    Leader {
        token: String,
        storage: Log,
        replication: Arc<Mutex<Replication>>,
    },
    Follower {
        storage: Log,
        leader_client: TokkiClient,
    },
}
impl AppState {
    pub fn storage(&self) -> &Log {
        match self {
            AppState::Leader { storage, .. } => storage,
            AppState::Follower { storage, .. } => storage,
        }
    }
}
