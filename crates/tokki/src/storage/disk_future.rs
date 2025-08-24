// use std::{
//     io,
//     pin::Pin,
//     task::{Context, Poll},
// };

// use tokki_common::{Offset, Record};

// use crate::storage::Storage;

// #[derive(Default, Clone)]
// enum StoredRecord {
//     #[default]
//     Empty,
//     Uncommitted(Record),
//     // Committed(Record),
//     // Aborted,
// }

// pub struct DisKFutureStorage {
//     // todo
// }

// impl DisKFutureStorage {
//     pub fn new() {}
// }

// struct StorageReactor {
//     big_array_of_bytes
// }

// struct PutRecordFuture {
//     completion_token: Option<CompletionToken>,
//     record: Record,
// }

// impl Future for PutRecordFuture {
//     type Output = io::Result<Offset>;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         todo!()
//     }
// }

// impl Storage for DisKFutureStorage {
//     // async fn max_offset(&self) -> io::Result<Option<Offset>> {
//     //     todo!()
//     // }

//     async fn max_offset(&self) -> io::Result<Option<Offset>> {
//         todo!()
//     }

//     fn put_record(&self, record: Record) -> impl Future<Output = io::Result<Offset>> {
//         PutRecordFuture {}
//     }

//     async fn get_records(
//         &self,
//         offset: Offset,
//         max_records: usize,
//     ) -> io::Result<(Vec<Record>, Offset)> {
//         todo!()
//     }
// }
