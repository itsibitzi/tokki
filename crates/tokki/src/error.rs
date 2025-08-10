use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Failed to bind to port: {port}"))]
    PortBind { port: u16, source: std::io::Error },
}
