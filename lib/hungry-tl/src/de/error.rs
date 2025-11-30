use std::str::Utf8Error;
use std::string::FromUtf8Error;

#[derive(Clone, Debug)]
pub enum Error {
    UnexpectedConstructor { id: u32 },
    UnexpectedEndOfBuffer,
    InvalidUtf8String(Utf8Error),
}

impl From<FromUtf8Error> for Error {
    #[inline]
    fn from(value: FromUtf8Error) -> Self {
        Self::InvalidUtf8String(value.utf8_error())
    }
}
