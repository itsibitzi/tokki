use common::Offset;
use tokio::sync::oneshot;

pub struct WaitingRequest {
    pub waiting_for: Offset,
    pub wake_tx: oneshot::Sender<()>,
}

impl Ord for WaitingRequest {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.waiting_for.cmp(&other.waiting_for)
    }
}

impl PartialOrd for WaitingRequest {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for WaitingRequest {}

impl PartialEq for WaitingRequest {
    fn eq(&self, other: &Self) -> bool {
        self.waiting_for == other.waiting_for
    }
}

impl WaitingRequest {
    pub fn wake(self) {
        _ = self.wake_tx.send(());
    }
}
