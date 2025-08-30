use std::{
    io,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use tokki_common::{Offset, Record};

use crate::storage::Storage;

const OFFSETS_SIZE: usize = 1024 * 1024 * 1024;
const SIZE: usize = 1024 * 1024 * 1024 * 1024;

#[derive(Clone)]
pub struct InMemoryLockFree {
    inner: Arc<InMemoryLockFreeInner>,
}

pub struct InMemoryLockFreeInner {
    offsets: Box<[usize; OFFSETS_SIZE]>,
    commited_offset_head: AtomicUsize,
    offset_head: AtomicUsize,
    data: Box<[u8; SIZE]>,
    committed_data_head: AtomicUsize,
    data_head: AtomicUsize,
}

impl InMemoryLockFree {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(InMemoryLockFreeInner {
                offsets: Box::new([0; OFFSETS_SIZE]),
                commited_offset_head: AtomicUsize::new(0),
                offset_head: AtomicUsize::new(0),
                data: Box::new([0; SIZE]),
                committed_data_head: AtomicUsize::new(0),
                data_head: AtomicUsize::new(0),
            }),
        }
    }
}

impl Storage for InMemoryLockFree {
    async fn max_offset(&self) -> io::Result<Option<Offset>> {
        let current_head = self.inner.commited_offset_head.load(Ordering::SeqCst);
        if current_head == 0 {
            Ok(None)
        } else {
            Ok(Some(Offset(current_head - 1)))
        }
    }

    async fn put_record(&self, record: Record) -> io::Result<Offset> {
        let inner = self.inner.as_ref();
        let serializd_len = record.serialized_len();

        // Reserve offset and data
        // Offset and data are not necessarily acquired in-order. E.g.
        // offset -> data start
        // 0 -> 0
        // 1 -> 200
        // 2 -> 100
        let offset_idx = inner.offset_head.fetch_add(1, Ordering::AcqRel);
        let offset_end = offset_idx + 1;
        if offset_idx >= OFFSETS_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "Offset buffer capacity exceeded",
            ));
        }

        let data_start = inner.data_head.fetch_add(serializd_len, Ordering::AcqRel);
        let data_end = data_start + serializd_len;
        if data_start >= OFFSETS_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "Data buffer capacity exceeded",
            ));
        }

        // Write the data
        let buf = unsafe {
            let base_ptr = inner.data.as_ptr();
            let start_ptr = base_ptr.add(data_start) as *mut u8;
            tracing::trace!(
                ?data_start,
                ?base_ptr,
                ?start_ptr,
                ?serializd_len,
                "Writing data"
            );
            std::slice::from_raw_parts_mut(start_ptr, serializd_len)
        };
        record.to_bytes(buf)?;

        unsafe {
            let ptr = inner.offsets.as_ptr().add(offset_idx) as *mut usize;
            *ptr = data_start;
        }

        // Advance the committed heads
        // Make sure the data is committed before the offset is
        loop {
            if let Ok(_) = inner.commited_offset_head.compare_exchange(
                offset_idx,
                offset_end,
                Ordering::Relaxed, // TODO fix
                Ordering::Relaxed, // TODO fix
            ) {
                break;
            }
        }

        loop {
            if let Ok(_) = inner.committed_data_head.compare_exchange(
                data_start,
                data_end,
                Ordering::Relaxed, // TODO fix
                Ordering::Relaxed, // TODO fix
            ) {
                break;
            }
        }

        Ok(Offset(offset_idx))
    }

    async fn get_records(
        &self,
        offset: Offset,
        max_records: usize,
    ) -> io::Result<(Vec<Record>, Offset)> {
        let inner = self.inner.as_ref();
        let start_offset = offset.0;
        let committed_head = inner.commited_offset_head.load(Ordering::SeqCst);

        if start_offset >= committed_head {
            return Ok((Vec::new(), offset));
        }

        let mut records = Vec::new();

        // Determine how many records we can read
        let available_records = committed_head - start_offset;
        let records_to_read = available_records.min(max_records);

        for current_offset in start_offset..start_offset + records_to_read {
            let data_pos = inner.offsets[current_offset];

            let buf = &inner.data[data_pos..];

            let (record, _) = Record::from_bytes(buf)?;

            records.push(record);
        }

        Ok((records, Offset(start_offset + records_to_read + 1)))
    }
}
