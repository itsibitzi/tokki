mod waiting_request;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use common::Offset;
use tokio::sync::oneshot;

use crate::replication::waiting_request::WaitingRequest;

pub struct Replication {
    waiting_requests: BinaryHeap<Reverse<WaitingRequest>>,
    required_replicas: usize,
    follower_max_offsets: HashMap<String, Offset>,
}

impl Replication {
    pub fn new(required_replicas: usize) -> Self {
        Self {
            required_replicas,
            waiting_requests: Default::default(),
            follower_max_offsets: Default::default(),
        }
    }

    pub fn register_wait(&mut self, waiting_for: Offset, wake_tx: oneshot::Sender<()>) {
        let waiting_request = WaitingRequest {
            waiting_for,
            wake_tx,
        };
        self.waiting_requests.push(Reverse(waiting_request));
        tracing::info!("Registered wait for {:?}", waiting_for);
    }

    pub fn update_follower_max_offset(&mut self, follower: String, offset: Option<Offset>) {
        if let Some(offset) = offset {
            self.follower_max_offsets.insert(follower, offset);

            while let Some(next_req) = self.waiting_requests.peek() {
                let followers_beyond = self
                    .follower_max_offsets
                    .values()
                    .filter(|o| **o >= next_req.0.waiting_for)
                    .count();

                if followers_beyond >= self.required_replicas {
                    let next_req = self.waiting_requests.pop().expect("Some waiting request");
                    next_req.0.wake();
                } else {
                    break;
                }
            }
        }
    }
}
