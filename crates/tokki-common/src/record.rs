use std::hash::BuildHasher as _;
use std::hash::Hasher as _;

use gxhash::{GxBuildHasher, GxHasher};
use hmac::digest::Update as _;
use serde::{Deserialize, Serialize};

use crate::hmac::HmacSha256;
use crate::hmac::HmacValue;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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

    pub fn serialized_len(&self) -> usize {
        size_of::<usize>() // Key length
            + self.key.len() // Key bytes
            + size_of::<usize>() // Value length
            + self.value.len() // Value bytes
            + size_of::<u64>() // Checksum
    }

    pub fn to_bytes(&self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        use std::io::{Cursor, Write};

        let mut cursor = Cursor::new(buf);

        cursor.write_all(&self.key.len().to_le_bytes())?;
        cursor.write_all(&self.key)?;
        cursor.write_all(&self.value.len().to_le_bytes())?;
        cursor.write_all(&self.value)?;
        cursor.write_all(&self.checksum.to_le_bytes())?;

        Ok(cursor.position() as usize)
    }

    pub fn from_bytes(buf: &[u8]) -> Result<(Self, usize), std::io::Error> {
        use std::io::{Cursor, Read};

        let mut cursor = Cursor::new(buf);

        let mut key_len_bytes = [0u8; size_of::<usize>()];
        cursor.read_exact(&mut key_len_bytes)?;
        let key_len = usize::from_le_bytes(key_len_bytes);

        let mut key = vec![0u8; key_len];
        cursor.read_exact(&mut key)?;

        let mut value_len_bytes = [0u8; size_of::<usize>()];
        cursor.read_exact(&mut value_len_bytes)?;
        let value_len = usize::from_le_bytes(value_len_bytes);

        let mut value = vec![0u8; value_len];
        cursor.read_exact(&mut value)?;

        let mut checksum_bytes = [0u8; 8];
        cursor.read_exact(&mut checksum_bytes)?;
        let checksum = u64::from_le_bytes(checksum_bytes);

        if Record::raw_checksum(&key, &value) != checksum {
            // TODO this should be an error
            panic!("Checksum fail");
        }

        let record = Self {
            key,
            value,
            checksum,
        };

        Ok((record, cursor.position() as usize))
    }
}

impl HmacValue for Record {
    fn update_mac(&self, mac: &mut HmacSha256) {
        mac.update(&self.checksum().to_le_bytes());
        mac.update(&self.key());
        mac.update(&self.value());
    }
}
