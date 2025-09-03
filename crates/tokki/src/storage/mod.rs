use std::io;

pub use in_memory::InMemoryStorage;
pub use in_memory_channel::InMemoryChannelStorage;
pub use in_memory_lockfree::InMemoryLockFree;
use tokki_common::{Offset, Record};

mod disk_future;
mod in_memory;
mod in_memory_channel;
mod in_memory_lockfree;

#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    /// Get the current maximum offset
    async fn max_offset(&self) -> io::Result<Option<Offset>>;

    /// Put a record on the log, returning it's offset.
    async fn put_record(&self, record: Record) -> io::Result<Offset>;

    /// Get `max_records` number off `Records` from the provided `offset`.
    async fn get_records(
        &self,
        offset: Offset,
        max_records: usize,
    ) -> io::Result<(Vec<Record>, Offset)>;
}
