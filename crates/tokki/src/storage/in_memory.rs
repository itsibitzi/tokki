use std::{
    io,
    sync::{Arc, Mutex},
};

use tokki_common::{Offset, Record};

use crate::storage::Storage;

#[derive(Default, Clone)]
pub struct InMemoryStorage {
    inner: Arc<Mutex<InMemoryStorageInner>>,
}

#[derive(Default)]
pub struct InMemoryStorageInner {
    records: Vec<StoredRecord>,
}

#[derive(Default, Clone)]
enum StoredRecord {
    #[default]
    Empty,
    Uncommitted(Record),
    // Committed(Record),
    // Aborted,
}

#[async_trait::async_trait]
impl Storage for InMemoryStorage {
    async fn max_offset(&self) -> io::Result<Option<Offset>> {
        let guard = self.inner.lock().expect("No panics");
        if guard.records.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Offset(guard.records.len() - 1)))
        }
    }

    async fn put_record(&self, record: Record) -> io::Result<Offset> {
        let mut guard = self.inner.lock().expect("No panics");
        guard.records.push(StoredRecord::Uncommitted(record));
        let offset = guard.records.len() - 1;
        tracing::debug!("Put record at {}", offset);
        Ok(Offset(offset))
    }

    /// Get some number of records from an offset. Returns a list of the records and the offset of the next record
    async fn get_records(
        &self,
        offset: Offset,
        max_records: usize,
    ) -> io::Result<(Vec<Record>, Offset)> {
        let guard = self.inner.lock().expect("No panics");
        let mut records = Vec::new();
        let mut last_offset = offset.0;

        for (index, record_opt) in guard
            .records
            .iter()
            .enumerate()
            .skip(offset.0)
            .take(max_records)
        {
            // TODO add distinction between uncommited and commited
            if let StoredRecord::Uncommitted(record) = record_opt {
                records.push(record.clone());
                last_offset = index;
            } else {
                break;
            }
        }

        let next_record_offset = last_offset + 1;
        Ok((records, Offset(next_record_offset)))
    }

    //     /// Write an uncomitted record to an exact location
    //     pub fn replicate_record(&self, offset: Offset, record: Record) {
    //         let mut guard = self.inner.lock().expect("No panics");
    //         let len = guard.records.len();
    //         guard
    //             .records
    //             .resize(offset.0.max(len) + 1, StoredRecord::Empty);

    //         match guard.records.get_mut(offset.0) {
    //             Some(stored_record) => match stored_record {
    //                 StoredRecord::Empty => {
    //                     *stored_record = StoredRecord::Uncommitted(record);
    //                 }
    //                 _ => {}
    //             },
    //             None => {
    //                 panic!("Should have just extended to offset: {}", offset.0)
    //             }
    //         }
    //         tracing::info!("Replicated record at {}", offset.0);
    //     }

    //     pub fn commit_record(&self, offset: Offset) {
    //         let mut guard = self.inner.lock().expect("No panics");
    //         match guard.records.get_mut(offset.0) {
    //             Some(stored_record) => {
    //                 match stored_record {
    //                     StoredRecord::Empty => {
    //                         tracing::warn!(offset = offset.0, "Commit attempted on empty record");
    //                     }
    //                     StoredRecord::Uncommitted(record) => {
    //                         let record = std::mem::take(record);
    //                         *stored_record = StoredRecord::Committed(record);
    //                     }
    //                     StoredRecord::Committed(_) => {
    //                         // no-op
    //                     }
    //                     StoredRecord::Aborted => {
    //                         tracing::warn!(offset = offset.0, "Commit attempted on aborted record");
    //                     }
    //                 }
    //             }
    //             None => {
    //                 tracing::warn!(offset = offset.0, "Commit attempted on empty record");
    //             }
    //         }
    //     }

    //     pub fn abort_record(&self, offset: Offset) {
    //         let mut guard = self.inner.lock().expect("No panics");
    //         match guard.records.get_mut(offset.0) {
    //             Some(stored_record) => match stored_record {
    //                 StoredRecord::Empty => {
    //                     tracing::warn!(offset = offset.0, "Abort attempted on empty record");
    //                     *stored_record = StoredRecord::Aborted;
    //                 }
    //                 StoredRecord::Uncommitted(_) => {
    //                     *stored_record = StoredRecord::Aborted;
    //                 }
    //                 StoredRecord::Committed(_) => {
    //                     *stored_record = StoredRecord::Aborted;
    //                 }
    //                 StoredRecord::Aborted => {
    //                     tracing::warn!(offset = offset.0, "Commit attempted on aborted record");
    //                 }
    //             },
    //             None => {
    //                 tracing::warn!(offset = offset.0, "Abort attempted on out");
    //             }
    //         }
    //     }
}
