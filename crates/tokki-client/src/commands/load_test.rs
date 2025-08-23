use futures::stream::{self, StreamExt};
use rand::{RngCore as _, SeedableRng};
use rand_chacha::ChaCha12Rng;
use tokki_api::{TokkiClient, put_record::PutRecordsRequest};
use tokki_common::Record;
use url::Url;

const PARALLELISM: usize = 64;
pub async fn load_test(base_url: Url, count: usize, batch_size: usize) {
    // Get baseline
    let batch_count = count / batch_size;
    let client = TokkiClient::new(base_url);

    stream::iter(0..batch_count)
        .map(|_| {
            let client = client.clone();
            async move { client.get_healthcheck().await.expect("Get healthcheck") }
        })
        .buffer_unordered(PARALLELISM)
        .collect::<Vec<_>>()
        .await;

    // Actual measurement
    let mut rng = ChaCha12Rng::seed_from_u64(1337);

    let batches = (0..batch_count)
        .map(|_| {
            let records = (0..batch_size)
                .map(|_| {
                    let mut bytes = [0u8; 32];
                    rng.fill_bytes(&mut bytes);
                    Record::new(bytes, bytes)
                })
                .collect();

            PutRecordsRequest::new(records)
        })
        .collect::<Vec<_>>();

    let start = std::time::Instant::now();

    stream::iter(batches)
        .map(|batch| {
            let client = client.clone();
            async move { client.put_record(batch).await.expect("Put records") }
        })
        .buffer_unordered(PARALLELISM)
        .collect::<Vec<_>>()
        .await;

    println!("Elapsed: {}ms", start.elapsed().as_millis());
}
