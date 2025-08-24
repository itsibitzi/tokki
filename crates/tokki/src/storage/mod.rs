mod disk_future;
mod in_memory;
mod in_memory_channel;

use std::io;

pub use in_memory::InMemoryStorage;
pub use in_memory_channel::InMemoryChannelStorage;
use tokki_common::{Offset, Record};

pub trait Storage {
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

#[derive(Clone)]
pub enum StorageEngine {
    InMemory(InMemoryStorage),
    InMemoryChannel(InMemoryChannelStorage),
}

impl StorageEngine {
    /// Get the current maximum offset
    pub async fn max_offset(&self) -> io::Result<Option<Offset>> {
        match self {
            StorageEngine::InMemory(in_memory_storage) => in_memory_storage.max_offset().await,
            StorageEngine::InMemoryChannel(in_memory_channel_storage) => {
                in_memory_channel_storage.max_offset().await
            }
        }
    }

    /// Put a record on the log, returning it's offset.
    pub async fn put_record(&self, record: Record) -> io::Result<Offset> {
        match self {
            StorageEngine::InMemory(in_memory_storage) => {
                in_memory_storage.put_record(record).await
            }
            StorageEngine::InMemoryChannel(in_memory_channel_storage) => {
                in_memory_channel_storage.put_record(record).await
            }
        }
    }

    /// Get `max_records` number off `Records` from the provided `offset`.
    pub async fn get_records(
        &self,
        offset: Offset,
        max_records: usize,
    ) -> io::Result<(Vec<Record>, Offset)> {
        match self {
            StorageEngine::InMemory(in_memory_storage) => {
                in_memory_storage.get_records(offset, max_records).await
            }
            StorageEngine::InMemoryChannel(in_memory_channel_storage) => {
                in_memory_channel_storage
                    .get_records(offset, max_records)
                    .await
            }
        }
    }
}
