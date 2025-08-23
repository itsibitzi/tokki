use std::str::FromStr;

use tokki_api::{TokkiClient, get_records::GetRecordsRequest, put_record::PutRecordsRequest};
use tokki_common::{Offset, Record};
use url::Url;

#[tokio::main]
async fn main() {
    let client_1 = TokkiClient::new(Url::from_str("http://127.0.0.1:7777").expect("Parse URL"));
    let client_2 = TokkiClient::new(Url::from_str("http://127.0.0.1:8888").expect("Parse URL"));
    let client_3 = TokkiClient::new(Url::from_str("http://127.0.0.1:9999").expect("Parse URL"));

    client_1
        .put_record(PutRecordsRequest::single(Record::new("foo", "bar")))
        .await
        .expect("Put record");

    let records = client_1
        .get_records(GetRecordsRequest::new(Offset::new(0), 10))
        .await
        .expect("Get records");

    println!("Records: {:?}", records);

    let records = client_2
        .get_records(GetRecordsRequest::new(Offset::new(0), 10))
        .await
        .expect("Get records");

    println!("Records: {:?}", records);

    let records = client_3
        .get_records(GetRecordsRequest::new(Offset::new(0), 10))
        .await
        .expect("Get records");

    println!("Records: {:?}", records);
}
