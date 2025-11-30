use std::fmt;

#[derive(Clone, Debug)]
pub struct Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("validation error")
    }
}

impl std::error::Error for Error {}
