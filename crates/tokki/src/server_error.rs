use snafu::Snafu;

/// Errors relating to starting the server
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ServerError {
    #[snafu(display("Failed to bind to port: {port}"))]
    PortBind { port: u16, source: std::io::Error },
    #[snafu(display("Failure to serve: {source}"))]
    Serve { source: std::io::Error },
}
