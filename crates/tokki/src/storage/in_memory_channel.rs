use std::{
    io,
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use tokio::{
    fs::File,
    sync::{
        mpsc::{Receiver, Sender, channel},
        oneshot,
    },
};
use tokki_common::{Offset, Record};

use crate::storage::Storage;

enum LogFileRequest {
    Put(Record),
    Get((Offset, usize)),
}

enum LogFileResponse {
    Put(io::Result<Offset>),
    Get(io::Result<(Vec<Record>, Offset)>),
}
#[derive(Default, Clone)]
enum StoredRecord {
    #[default]
    Empty,
    Uncommitted(Record),
    // Committed(Record),
    // Aborted,
}
struct LogFile {
    cmd_rx: Receiver<(LogFileRequest, oneshot::Sender<LogFileResponse>)>,
    records: Vec<StoredRecord>,
}

impl LogFile {
    fn new(cmd_rx: Receiver<(LogFileRequest, oneshot::Sender<LogFileResponse>)>) -> Self {
        Self {
            cmd_rx,
            records: Default::default(),
        }
    }

    async fn run(&mut self) {
        loop {
            match self.cmd_rx.recv().await {
                Some((request, res_tx)) => match request {
                    LogFileRequest::Put(record) => {
                        let offset = Offset::new(self.records.len());
                        self.records.push(StoredRecord::Uncommitted(record));
                        let _ = res_tx.send(LogFileResponse::Put(Ok(offset)));
                    }
                    LogFileRequest::Get((offset, max_records)) => {
                        let start = offset.0;
                        let end = (start + max_records).min(self.records.len());

                        let mut records = Vec::new();
                        for i in start..end {
                            match &self.records[i] {
                                StoredRecord::Uncommitted(record) => {
                                    records.push(record.clone());
                                }
                                StoredRecord::Empty => {
                                    break;
                                }
                            }
                        }

                        let next_offset = if end < self.records.len() {
                            Offset::new(end)
                        } else {
                            Offset::new(self.records.len())
                        };

                        let _ = res_tx.send(LogFileResponse::Get(Ok((records, next_offset))));
                    }
                },
                None => break,
            }
        }
    }
}

#[derive(Clone)]
pub struct InMemoryChannelStorage {
    max_offset: Arc<AtomicUsize>,
    cmd_tx: Sender<(LogFileRequest, oneshot::Sender<LogFileResponse>)>,
}

impl InMemoryChannelStorage {
    pub async fn new() -> io::Result<Self> {
        let (cmd_tx, cmd_rx) = channel(1024);

        let mut logger = LogFile::new(cmd_rx);
        tokio::spawn(async move {
            logger.run().await;
        });

        Ok(Self {
            max_offset: Arc::new(AtomicUsize::default()),
            cmd_tx,
        })
    }
}

impl Storage for InMemoryChannelStorage {
    async fn max_offset(&self) -> io::Result<Option<Offset>> {
        let max_offset = self.max_offset.load(Ordering::Relaxed);

        if max_offset == 0 {
            Ok(None)
        } else {
            Ok(Some(Offset::new(max_offset - 1)))
        }
    }

    async fn put_record(&self, record: Record) -> io::Result<Offset> {
        let (res_tx, res_rx) = oneshot::channel();
        self.cmd_tx
            .send((LogFileRequest::Put(record), res_tx))
            .await
            .unwrap();

        let res = match res_rx.await.unwrap() {
            LogFileResponse::Put(result) => result?,
            _ => unreachable!(),
        };

        Ok(res)
    }

    async fn get_records(
        &self,
        offset: Offset,
        max_records: usize,
    ) -> io::Result<(Vec<Record>, Offset)> {
        let (res_tx, res_rx) = oneshot::channel();

        self.cmd_tx
            .send((LogFileRequest::Get((offset, max_records)), res_tx))
            .await
            .unwrap();

        let res = match res_rx.await.unwrap() {
            LogFileResponse::Get(result) => result?,
            _ => unreachable!(),
        };

        Ok(res)
    }
}
