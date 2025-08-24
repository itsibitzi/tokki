use std::time::Instant;

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

    let start = Instant::now();
    stream::iter(0..batch_count)
        .map(|_| {
            let start = Instant::now();
            let client = client.clone();
            async move {
                client.get_healthcheck().await.expect("Get healthcheck");
                start.elapsed()
            }
        })
        .buffer_unordered(PARALLELISM)
        .collect::<Vec<_>>()
        .await;
    println!("Baseline: {}ms", start.elapsed().as_millis());

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

    let start = Instant::now();
    let batch_times = stream::iter(batches)
        .map(|batch| {
            let start = Instant::now();
            let client = client.clone();
            async move {
                client.put_record(batch).await.expect("Put records");
                start.elapsed()
            }
        })
        .buffer_unordered(PARALLELISM)
        .collect::<Vec<_>>()
        .await;
    let elapsed_times = start.elapsed();

    let total_ms = elapsed_times.as_millis() as f64;

    let avg_batch_time = total_ms / batch_times.len() as f64;

    println!("Statistics:");
    println!("  Total batches: {}", batch_times.len());
    println!("  Records per batch: {}", batch_size);
    println!("  Total records: {}", count);
    println!("  Total time: {}ms", start.elapsed().as_millis());
    println!("  Average batch time: {:.2}ms", avg_batch_time);
    println!(
        "  Throughput: {:.2} batches/sec",
        1000.0 * batch_count as f64 / total_ms
    );
    println!(
        "  Throughput: {:.2} records/sec",
        1000.0 * count as f64 / total_ms
    );
}
