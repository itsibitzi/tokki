use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum HmacError {
    #[snafu(display("Provided and computed HMAC did not match"))]
    Mismatch,
}
