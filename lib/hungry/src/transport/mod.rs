mod error;
mod full;

use std::ops::{Range, RangeFrom};

use bytes::BytesMut;

use crate::{Envelope, EnvelopeSize};

pub(self) use error::unpack_err;

pub use error::{UnpackError, UnpackErrorKind};
pub use full::Full;

#[derive(Debug, Eq, PartialEq)]
pub enum Unpack {
    Envelope { data: Range<usize>, next: usize },
    QuickAck { token: u32, len: usize },
}

pub trait Transport: EnvelopeSize {
    type Read: TransportRead<Transport = Self>;
    type Write: TransportWrite<Transport = Self>;

    fn split(self) -> (Self::Read, Self::Write);
}

pub trait TransportRead: Unpin {
    type Transport: Transport;

    fn length(&mut self, buffer: &mut [u8]) -> usize;

    fn unpack(&mut self, buffer: &mut [u8]) -> Result<Unpack, UnpackError>;
}

pub trait TransportWrite: Unpin {
    type Transport: Transport;

    fn pack(
        &mut self,
        buffer: &mut BytesMut,
        envelope: Envelope<Self::Transport>,
    ) -> RangeFrom<usize>;
}
