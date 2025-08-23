use hmac::Mac;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::hmac::{HmacError, HmacSha256, HmacValue};

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct HmacForm<T> {
    #[serde_as(as = "serde_with::hex::Hex")]
    hmac: Vec<u8>,
    data: T,
}

impl<T> HmacForm<T>
where
    T: HmacValue,
{
    pub fn new(data: T, token: &str) -> Self {
        let mut mac =
            HmacSha256::new_from_slice(token.as_bytes()).expect("HMAC can take key of any size");
        data.update_mac(&mut mac);
        let hmac = mac.finalize();
        let hmac = hmac.into_bytes().to_vec();

        Self { hmac, data }
    }

    pub fn into_verified(self, token: &str) -> Result<T, HmacError> {
        tracing::debug!("Checking HMAC form");
        let mut mac =
            HmacSha256::new_from_slice(token.as_bytes()).expect("HMAC can take key of any size");
        self.data.update_mac(&mut mac);
        let hmac = mac.finalize();
        let hmac = hmac.into_bytes();
        if hmac.as_slice() != self.hmac.as_slice() {
            return Err(HmacError::Mismatch);
        }
        Ok(self.data)
    }
}
