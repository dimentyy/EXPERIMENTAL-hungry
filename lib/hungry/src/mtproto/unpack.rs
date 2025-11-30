use std::fmt;
use std::num::NonZeroI64;

use crate::crypto::aes_ige_decrypt;
use crate::mtproto::{AuthKey, Side};
use crate::utils::SliceExt;

#[derive(Debug)]
pub enum Message {
    Plain(PlainMessage),
    Encrypted(EncryptedMessage),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::Plain(message) => message.fmt(f),
            Message::Encrypted(message) => message.fmt(f),
        }
    }
}

impl Message {
    pub fn unpack(buffer: &[u8]) -> Message {
        assert!(buffer.len() >= 20);

        let Some(auth_key_id) = NonZeroI64::new(i64::from_le_bytes(*buffer[0..8].arr())) else {
            let message_id = i64::from_le_bytes(*buffer[8..16].arr());
            let message_length = i32::from_le_bytes(*buffer[16..20].arr());

            return Message::Plain(PlainMessage {
                message_id,
                message_length,
            });
        };

        let message_key = *buffer[8..24].arr();

        Message::Encrypted(EncryptedMessage {
            auth_key_id,
            message_key,
        })
    }
}

#[derive(Debug)]
pub struct PlainMessage {
    pub message_id: i64,
    pub message_length: i32,
}

impl fmt::Display for PlainMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "plain message: id=0x{:08x}, length={}",
            self.message_id, self.message_length
        )
    }
}

#[derive(Debug)]
pub struct EncryptedMessage {
    pub auth_key_id: NonZeroI64,
    pub message_key: [u8; 16],
}

impl fmt::Display for EncryptedMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "encrypted message: auth_key_id=0x{:016x}, message_key={:?}",
            self.auth_key_id.get(),
            self.message_key
        )
    }
}

#[derive(Debug)]
pub struct DecryptedMessage {
    pub salt: i64,
    pub session_id: i64,
    pub message_id: i64,
    pub seq_no: i64,
    pub message_length: i64,
}

impl EncryptedMessage {
    pub fn unpack_encrypted(self, auth_key: &AuthKey, buffer: &mut [u8]) -> DecryptedMessage {
        assert!(buffer.len() >= 64);

        assert_eq!(self.auth_key_id.get(), i64::from_le_bytes(*auth_key.id()));
        assert_eq!(&self.message_key, buffer[8..24].arr());

        let (aes_key, aes_iv) = auth_key.compute(&self.message_key, Side::Server);

        aes_ige_decrypt(&mut buffer[24..], &aes_key, &aes_iv);

        let salt = i64::from_le_bytes(*buffer[24..32].arr());
        let session_id = i64::from_le_bytes(*buffer[32..40].arr());
        let message_id = i64::from_le_bytes(*buffer[40..48].arr());
        let seq_no = i64::from_le_bytes(*buffer[48..56].arr());
        let message_length = i64::from_le_bytes(*buffer[56..64].arr());

        DecryptedMessage {
            salt,
            session_id,
            message_id,
            seq_no,
            message_length,
        }
    }
}
