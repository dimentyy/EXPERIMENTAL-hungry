mod queued;

use tokio::io::AsyncWrite;

use crate::transport::Transport;

pub use queued::QueuedWriter;

pub struct Writer<W: AsyncWrite + Unpin, T: Transport> {
    driver: W,
    transport: T::Write,
}

impl<W: AsyncWrite + Unpin, T: Transport> Writer<W, T> {
    pub(crate) fn new(driver: W, transport: T::Write) -> Self {
        Self { driver, transport }
    }
}
