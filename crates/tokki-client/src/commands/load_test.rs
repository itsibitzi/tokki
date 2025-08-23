use rand::{RngCore as _, SeedableRng};
use rand_chacha::ChaCha12Rng;
use rayon::prelude::*;
use tokki_api::{TokkiClient, put_record::PutRecordsRequest};
use tokki_common::Record;
use url::Url;

pub async fn load_test(base_url: Url, count: usize, batch_size: usize) {
    let mut rng = ChaCha12Rng::seed_from_u64(1337);

    let batch_count = count / batch_size;
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

    let client = TokkiClient::new(base_url);

    let start = std::time::Instant::now();

    for batch in batches {
        client
            .put_record(batch)
            .await
            .expect("Failed to put record batch");
    }

    println!("Elapsed: {}ms", start.elapsed().as_millis());
}
