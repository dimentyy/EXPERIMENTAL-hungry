use std::marker::PhantomData;

use bytes::BytesMut;

use crate::tl::de;
use crate::transport::{Packet, QuickAck, Unpack};
use crate::{mtproto, reader};

#[derive(Debug)]
pub enum PlainDeserializationError {
    QuickAck(QuickAck),
    EncryptedMessage(mtproto::EncryptedMessage),
    Deserialization(de::Error),
}

impl From<de::Error> for PlainDeserializationError {
    fn from(value: de::Error) -> Self {
        Self::Deserialization(value)
    }
}

pub struct DeserializePlain<T: de::Deserialize + Unpin>(PhantomData<T>);

impl<T: de::Deserialize + Unpin> DeserializePlain<T> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: de::Deserialize + Unpin> reader::HandleOutput for DeserializePlain<T> {
    type Output = Result<T, PlainDeserializationError>;

    fn acquired(&mut self, buffer: &mut BytesMut, unpack: Unpack) -> Self::Output {
        let (data, next) = match unpack {
            Unpack::Packet(Packet { data, next }) => (data, next),
            Unpack::QuickAck(quick_ack) => {
                unsafe { buffer.set_len(0) };

                return Err(PlainDeserializationError::QuickAck(quick_ack));
            }
        };

        let message = match mtproto::Message::unpack(&mut buffer[data.clone()]) {
            mtproto::Message::Plain(message) => message,
            mtproto::Message::Encrypted(message) => {
                unsafe { buffer.set_len(0) };

                return Err(PlainDeserializationError::EncryptedMessage(message));
            }
        };

        let mut buf = de::Buf::new(&buffer[data.start + 20..data.end]);
        let value = T::deserialize_checked(&mut buf)?;

        unsafe { buffer.set_len(0) };

        Ok(value)
    }
}
