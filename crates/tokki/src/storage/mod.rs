mod disk;
mod in_memory;

pub use in_memory::InMemoryStorage;
use tokki_common::{Offset, Record};

pub trait Storage {
    /// Get the current maximum offset
    fn max_offset(&self) -> Option<Offset>;

    /// Put a record on the log, returning it's offset.
    fn put_record(&self, record: &Record) -> Offset;

    /// Get `max_records` number off `Records` from the provided `offset`.
    fn get_records(&self, offset: Offset, max_records: usize) -> (Vec<Record>, Offset);
}
