use std::hash::BuildHasher as _;
use std::hash::Hasher as _;

use gxhash::{GxBuildHasher, GxHasher};
use hmac::digest::Update as _;
use serde::{Deserialize, Serialize};

use crate::hmac::HmacSha256;
use crate::hmac::HmacValue;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Record {
    key: Vec<u8>,
    value: Vec<u8>,
    checksum: u64,
}

impl Record {
    pub fn new(key: impl Into<Vec<u8>>, value: impl Into<Vec<u8>>) -> Self {
        let key = key.into();
        let value = value.into();
        let checksum = Self::raw_checksum(&key, &value);
        Self {
            key,
            value,
            checksum,
        }
    }

    pub fn checksum(&self) -> u64 {
        Self::raw_checksum(&self.key, &self.value)
    }

    fn raw_checksum(key: &[u8], value: &[u8]) -> u64 {
        let mut hasher = GxHasher::default();
        hasher.write(key);
        hasher.write(value);
        hasher.finish()
    }

    pub fn hash_key(&self) -> u64 {
        GxBuildHasher::default().hash_one(&self.key)
    }

    pub fn key(&self) -> &[u8] {
        &self.key
    }

    pub fn value(&self) -> &[u8] {
        &self.value
    }
}

impl HmacValue for Record {
    fn update_mac(&self, mac: &mut HmacSha256) {
        mac.update(&self.checksum().to_le_bytes());
        mac.update(&self.key());
        mac.update(&self.value());
    }
}
