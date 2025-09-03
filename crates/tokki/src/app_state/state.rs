use std::sync::{Arc, Mutex};
use tokio::task::{JoinError, JoinHandle};
use tokki_api::TokkiClient;

use crate::{app_state::builder::AppStateBuilder, replication::Replication, storage::Storage};

#[derive(Clone)]
pub struct AppState {
    pub profiling_enabled: bool,
    pub inner: Arc<AppStateInner>,
}

pub enum AppStateInner {
    Leader {
        token: String,
        storage: Arc<dyn Storage>,
        replication: Arc<Mutex<Replication>>,
        required_replicas: usize,
    },
    Follower {
        token: String,
        storage: Arc<dyn Storage>,
        leader_client: TokkiClient,
        leader_poll_task: JoinHandle<Result<(), JoinError>>,
    },
}

impl AppState {
    pub fn builder() -> AppStateBuilder {
        AppStateBuilder {}
    }

    pub fn storage(&self) -> &dyn Storage {
        match self.inner.as_ref() {
            AppStateInner::Leader { storage, .. } => storage.as_ref(),
            AppStateInner::Follower { storage, .. } => storage.as_ref(),
        }
    }
}
