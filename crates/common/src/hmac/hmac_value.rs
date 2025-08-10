use hmac::digest::Update as _;

use crate::hmac::HmacSha256;

pub trait HmacValue {
    fn update_mac(&self, mac: &mut HmacSha256);
}

impl HmacValue for String {
    fn update_mac(&self, mac: &mut HmacSha256) {
        mac.update(self.as_bytes());
    }
}

impl<T> HmacValue for Option<T>
where
    T: HmacValue,
{
    fn update_mac(&self, mac: &mut HmacSha256) {
        match self {
            Some(v) => {
                mac.update(&[1]);
                v.update_mac(mac);
            }
            None => {
                mac.update(&[0]);
            }
        }
    }
}
