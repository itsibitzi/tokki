// use std::{
//     marker::PhantomData,
//     sync::{Arc, Mutex},
// };

// use crate::{
//     app_state::{AppState, state::AppStateInner},
//     replication::Replication,
//     storage::Storage,
// };

// pub struct AppStateBuilder {}

// impl AppStateBuilder {
//     pub fn leader<S: Storage>(self) -> LeaderBuilder<Unset, Unset, S> {
//         LeaderBuilder {
//             token: String::default(),
//             storage: None,
//             required_replicas: 0,
//             profiling_enabled: false,
//             marker: PhantomData,
//         }
//     }

//     pub fn follower(self) -> FollowerBuilder {
//         FollowerBuilder::default()
//     }
// }

// #[derive(Debug, Clone, Copy)]
// pub struct Set;

// #[derive(Debug, Clone, Copy)]
// pub struct Unset;

// pub struct LeaderBuilder<TokenStatus, StorageStatus, S: Storage> {
//     token: String,
//     storage: Option<S>,
//     required_replicas: usize,
//     profiling_enabled: bool,
//     marker: PhantomData<(TokenStatus, StorageStatus)>,
// }

// impl<StorageStaus, S: Storage> LeaderBuilder<Unset, StorageStaus, S> {
//     pub fn with_token(mut self, token: impl Into<String>) -> Self {
//         self.token = token.into();
//         self
//     }
// }

// impl<TokenStatus, S: Storage> LeaderBuilder<TokenStatus, Unset, S> {
//     pub fn with_storage(mut self, storage: S) -> LeaderBuilder<TokenStatus, Set, S> {
//         self.storage = Some(storage);
//         self
//     }
// }

// impl<TokenStatus, StorageStatus, S: Storage> LeaderBuilder<TokenStatus, StorageStatus, S> {
//     pub fn with_profiling_enabled(mut self, profiling_enabled: bool) -> Self {
//         self.profiling_enabled = profiling_enabled;
//         self
//     }
// }

// impl<S: Storage> LeaderBuilder<Set, Set, S> {
//     pub fn build(self) -> AppState {
//         AppState {
//             profiling_enabled: self.enable_profiling,
//             inner: Arc::new(AppStateInner::Leader {
//                 token: self.token,
//                 storage: self.storage.unwrap(),
//                 required_replicas: self.required_replicas,
//                 replication: Arc::new(Mutex::new(Replication::new(self.required_replicas))),
//             }),
//         }
//     }
// }

// #[derive(Default)]
// pub struct FollowerBuilder {}

// impl FollowerBuilder {}
