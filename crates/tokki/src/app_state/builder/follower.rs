use std::{marker::PhantomData, net::SocketAddr, sync::Arc, time::Duration};

use tokio::time::sleep;
use tokki_api::{TokkiClient, clustering::ReplicateLogRequest};
use url::Url;

use crate::{
    app_state::{
        AppState, AppStateInner,
        builder::{Set, Unset},
    },
    storage::Storage,
};

#[derive(Default)]
pub struct FollowerBuilder<A, T, S, L> {
    addr: Option<SocketAddr>,
    token: String,
    storage: Option<Arc<dyn Storage>>,
    profiling_enabled: bool,
    leader: Option<Url>,
    marker: PhantomData<(A, T, S, L)>,
}

impl<A, T, S, L> FollowerBuilder<A, T, S, L> {
    pub fn with_profiling_enabled(mut self, profiling_enabled: bool) -> Self {
        self.profiling_enabled = profiling_enabled;
        self
    }
}
impl<T, S, L> FollowerBuilder<Unset, T, S, L> {
    pub fn with_socket_addr(self, addr: SocketAddr) -> FollowerBuilder<Set, T, S, L> {
        FollowerBuilder {
            addr: Some(addr),
            token: self.token,
            storage: self.storage,
            profiling_enabled: self.profiling_enabled,
            leader: self.leader,
            marker: PhantomData,
        }
    }
}

impl<A, S, L> FollowerBuilder<A, Unset, S, L> {
    pub fn with_token(self, token: impl Into<String>) -> FollowerBuilder<A, Set, S, L> {
        FollowerBuilder {
            addr: self.addr,
            token: token.into(),
            storage: self.storage,
            profiling_enabled: self.profiling_enabled,
            leader: self.leader,
            marker: PhantomData,
        }
    }
}

impl<A, T, L> FollowerBuilder<A, T, Unset, L> {
    pub fn with_storage(self, storage: Arc<dyn Storage>) -> FollowerBuilder<A, T, Set, L> {
        FollowerBuilder {
            addr: self.addr,
            token: self.token,
            storage: Some(storage),
            profiling_enabled: self.profiling_enabled,
            leader: self.leader,
            marker: PhantomData,
        }
    }
}

impl<A, T, S> FollowerBuilder<A, T, S, Unset> {
    pub fn with_leader(self, leader: Url) -> FollowerBuilder<A, T, S, Set> {
        FollowerBuilder {
            addr: self.addr,
            token: self.token,
            storage: self.storage,
            profiling_enabled: self.profiling_enabled,
            leader: Some(leader),
            marker: PhantomData,
        }
    }
}

impl FollowerBuilder<Set, Set, Set, Set> {
    pub fn build(self) -> AppState {
        let addr = self.addr.unwrap();
        let token = self.token;
        let storage = self.storage.unwrap();
        let leader_client = TokkiClient::new(self.leader.unwrap());

        let leader_poll_task = tokio::task::spawn({
            let follower_url = format!("http://{}", addr);
            let token = token.clone();
            let storage = storage.clone();
            let leader_client = leader_client.clone();

            let mut backoff_ms = 100;
            async move {
                loop {
                    let req = ReplicateLogRequest::new(
                        follower_url.clone(),
                        storage.max_offset().await.expect("Get max offset"),
                    );

                    let res = leader_client
                        .replicate_records(req, &token)
                        .await
                        .expect("Send request")
                        .into_verified(&token)
                        .expect("good token");

                    if res.records.is_empty() {
                        if backoff_ms < 1000 {
                            backoff_ms += 10;
                        }
                    } else {
                        for r in res.records {
                            storage.put_record(r).await.expect("put record");
                        }
                        backoff_ms = 10;
                    }
                    sleep(Duration::from_millis(backoff_ms)).await;
                }
            }
        });

        AppState {
            profiling_enabled: self.profiling_enabled,
            inner: Arc::new(AppStateInner::Follower {
                token,
                storage,
                leader_client,
                leader_poll_task,
            }),
        }
    }
}
