use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crate::{
    app_state::{
        AppState,
        builder::{Set, Unset},
        state::AppStateInner,
    },
    replication::Replication,
    storage::Storage,
};

#[derive(Default)]
pub struct LeaderBuilder<TokenStatus, StorageStatus> {
    token: String,
    storage: Option<Arc<dyn Storage>>,
    required_replicas: usize,
    profiling_enabled: bool,
    marker: PhantomData<(TokenStatus, StorageStatus)>,
}

impl<TokenStatus, StorageStatus> LeaderBuilder<TokenStatus, StorageStatus> {
    pub fn with_profiling_enabled(mut self, profiling_enabled: bool) -> Self {
        self.profiling_enabled = profiling_enabled;
        self
    }

    pub fn with_required_replicas(mut self, required_replicas: usize) -> Self {
        self.required_replicas = required_replicas;
        self
    }
}

impl<S> LeaderBuilder<Unset, S> {
    pub fn with_token(mut self, token: impl Into<String>) -> LeaderBuilder<Set, S> {
        self.token = token.into();
        LeaderBuilder {
            token: self.token,
            storage: self.storage,
            required_replicas: self.required_replicas,
            profiling_enabled: self.profiling_enabled,
            marker: PhantomData,
        }
    }
}

impl<T> LeaderBuilder<T, Unset> {
    pub fn with_storage(self, storage: Arc<dyn Storage>) -> LeaderBuilder<T, Set> {
        LeaderBuilder {
            token: self.token,
            storage: Some(storage),
            required_replicas: self.required_replicas,
            profiling_enabled: self.profiling_enabled,
            marker: PhantomData,
        }
    }
}

impl LeaderBuilder<Set, Set> {
    pub fn build(self) -> AppState {
        AppState {
            profiling_enabled: self.profiling_enabled,
            inner: Arc::new(AppStateInner::Leader {
                token: self.token,
                storage: self.storage.unwrap(),
                required_replicas: self.required_replicas,
                replication: Arc::new(Mutex::new(Replication::new(self.required_replicas))),
            }),
        }
    }
}
