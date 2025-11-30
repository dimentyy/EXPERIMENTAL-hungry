use std::marker::PhantomData;

use bytes::BytesMut;

use crate::tl::de;
use crate::transport::Unpack;
use crate::{mtproto, reader};

#[derive(Debug)]
pub enum PlainDeserializerError {
    QuickAck { token: u32, len: usize },
    EncryptedMessage(mtproto::EncryptedMessage),
    Deserialization(de::Error),
}

impl From<de::Error> for PlainDeserializerError {
    fn from(value: de::Error) -> Self {
        Self::Deserialization(value)
    }
}

pub struct PlainDeserializer<T: de::Deserialize + Unpin>(PhantomData<T>);

impl<T: de::Deserialize + Unpin> PlainDeserializer<T> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: de::Deserialize + Unpin> reader::ReaderBehaviour for PlainDeserializer<T> {
    type Unpack = Result<T, PlainDeserializerError>;

    fn required(&mut self, buffer: &mut BytesMut, length: usize) {
        buffer.reserve(buffer.capacity() - length);
    }

    fn acquired(&mut self, buffer: &mut BytesMut, unpack: Unpack) -> Self::Unpack {
        let (data, next) = match unpack {
            Unpack::Envelope { data, next } => (data, next),
            Unpack::QuickAck { token, len } => {
                unsafe { buffer.set_len(0) };

                return Err(PlainDeserializerError::QuickAck { token, len });
            }
        };

        let message = match mtproto::Message::unpack(&mut buffer[data.clone()]) {
            mtproto::Message::Plain(message) => message,
            mtproto::Message::Encrypted(message) => {
                unsafe { buffer.set_len(0) };

                return Err(PlainDeserializerError::EncryptedMessage(message));
            }
        };

        let mut buf = de::Buf::new(&buffer[data.start + 20..data.end]);
        let value = T::deserialize_checked(&mut buf)?;

        unsafe { buffer.set_len(0) };

        Ok(value)
    }
}
