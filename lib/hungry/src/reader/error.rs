use std::{error, fmt, io};

use crate::transport::UnpackError;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Unpack(UnpackError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("reader error: ")?;

        match self {
            Error::Io(err) => err.fmt(f),
            Error::Unpack(err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<UnpackError> for Error {
    fn from(value: UnpackError) -> Self {
        Self::Unpack(value)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Io(err) => err,
            Error::Unpack(err) => err,
        })
    }
}
