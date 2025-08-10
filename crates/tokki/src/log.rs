use std::sync::{Arc, Mutex};

use common::{Offset, Record};

#[derive(Default, Clone)]
pub struct Log {
    inner: Arc<Mutex<StorageInner>>,
}

#[derive(Default)]
pub struct StorageInner {
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

impl Log {
    pub fn max_offset(&self) -> Option<Offset> {
        let guard = self.inner.lock().expect("No panics");
        if guard.records.is_empty() {
            None
        } else {
            Some(Offset(guard.records.len() - 1))
        }
    }

    /// Put a record on the log, returning it's offset.
    pub fn put_record(&self, record: &Record) -> Offset {
        let mut guard = self.inner.lock().expect("No panics");
        guard
            .records
            .push(StoredRecord::Uncommitted(record.clone()));
        let offset = guard.records.len() - 1;
        tracing::info!("Put record at {}", offset);
        Offset(offset)
    }

    /// Get some number of records from an offset. Returns a list of the records and the offset of the next record
    pub fn get_records(&self, offset: Offset, max_records: usize) -> (Vec<Record>, Offset) {
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
        (records, Offset(next_record_offset))
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
