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
use crate::writer::{write_zero_err, Writer};

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
        mtproto::pack_plain(&mut buffer, mtp, message_id);

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
        mtproto::pack_encrypted(&mut buffer, mtp, auth_key, salt, session_id);

        self.queue_impl(buffer, transport);
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<BytesMut>> {
        // Buffers may be returned out of order due to multiple being queued at the same time.
        if let Some(buffer) = self.ready.pop_front() {
            return Poll::Ready(Ok(buffer));
        }

        let Some(buffer) = self.buffers.front_mut() else {
            return Poll::Pending;
        };

        // Loop may not be used here because a written buffer will be lost due to an error.
        // Storing io::Error in the Writer to return in the next poll would be an overkill.
        let ready = match ready_ok!(pin!(&mut self.writer.driver).poll_write(cx, buffer.as_ref())) {
            0 => write_zero_err!(),
            n if n == buffer.len() => self.buffers.pop_front().unwrap(),
            n => buffer.split_to(n),
        };

        Poll::Ready(Ok(ready))
    }
}

impl<W: AsyncWrite + Unpin, T: Transport> Future for QueuedWriter<W, T> {
    type Output = io::Result<BytesMut>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().poll(cx)
    }
}
