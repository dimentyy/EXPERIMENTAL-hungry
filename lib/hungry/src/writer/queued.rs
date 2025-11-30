use std::collections::VecDeque;
use std::io;
use std::pin::{pin, Pin};
use std::task::{ready, Context, Poll};

use bytes::BytesMut;
use tokio::io::AsyncWrite;

use crate::envelope::Envelope;
use crate::mtproto;
use crate::transport::{Transport, TransportWrite};
use crate::utils::ready_ok;
use crate::writer::Writer;

pub struct QueuedWriter<W: AsyncWrite + Unpin, T: Transport> {
    ready: VecDeque<BytesMut>,
    writer: Writer<W, T>,
    buffers: VecDeque<BytesMut>,
}

impl<W: AsyncWrite + Unpin, T: Transport> QueuedWriter<W, T> {
    pub fn new(writer: Writer<W, T>) -> Self {
        Self {
            ready: VecDeque::new(),
            writer,
            buffers: VecDeque::new(),
        }
    }

    fn queue_impl(&mut self, mut buffer: BytesMut, envelope: Envelope<T>) {
        let packed = self.writer.transport.pack(&mut buffer, envelope);

        if packed.start > 0 {
            self.ready.push_back(buffer.split_to(packed.start));
        }

        self.buffers.push_back(buffer);
    }

    pub fn queue_plain(
        &mut self,
        mut buffer: BytesMut,
        transport: Envelope<T>,
        mtp: mtproto::PlainEnvelope,
        message_id: i64,
    ) {
        mtproto::pack::plain(&mut buffer, mtp, message_id);

        self.queue_impl(buffer, transport);
    }

    pub fn queue(
        &mut self,
        mut buffer: BytesMut,
        transport: Envelope<T>,
        mtp: mtproto::Envelope,
        auth_key: &mtproto::AuthKey,
        salt: i64,
        session_id: i64,
    ) {
        mtproto::pack::encrypted(&mut buffer, mtp, auth_key, salt, session_id);

        self.queue_impl(buffer, transport);
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<BytesMut>> {
        if let Some(buffer) = self.ready.pop_front() {
            return Poll::Ready(Ok(buffer));
        }

        let Some(buffer) = self.buffers.front_mut() else {
            return Poll::Pending;
        };

        let ready = match ready_ok!(pin!(&mut self.writer.driver).poll_write(cx, buffer.as_ref())) {
            0 => Err(io::Error::new(io::ErrorKind::WriteZero, "wrote 0 bytes")),
            n if n == buffer.len() => Ok(self.buffers.pop_front().unwrap()),
            n => Ok(buffer.split_to(n)),
        };

        if let Ok(buffer) = ready.as_ref() {
            crate::utils::dump(buffer.as_ref(), "WROTE");
        }

        Poll::Ready(ready)
    }
}

impl<W: AsyncWrite + Unpin, T: Transport> Future for QueuedWriter<W, T> {
    type Output = io::Result<BytesMut>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().poll(cx)
    }
}
