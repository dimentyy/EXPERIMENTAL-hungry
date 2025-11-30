use std::fmt;

macro_rules! unpack_err {
    ($kind:ident $( $args:tt )*) => {
        return Err(UnpackError {
            kind: UnpackErrorKind::$kind $( $args )*,
        })
    };
}

pub(super) use unpack_err;

#[derive(Clone, Debug)]
pub struct UnpackError {
    pub kind: UnpackErrorKind,
}

impl fmt::Display for UnpackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("transport unpack error: ")?;

        self.kind.fmt(f)
    }
}

impl std::error::Error for UnpackError {}

#[derive(Clone, Debug)]
pub enum UnpackErrorKind {
    QuickAck,
    Status(i32),
    BadLen(i32),
    BadCrc { received: u32, computed: u32 },
    BadSeq { received: i32, expected: i32 },
}

impl fmt::Display for UnpackErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use UnpackErrorKind::*;

        match self {
            QuickAck => write!(f, "quick ack is not supported"),
            Status(code) => write!(f, "status code: {code}"),
            BadLen(len) => write!(f, "bad len: {len}"),
            BadCrc {
                received: r,
                computed: c,
            } => write!(f, "bad crc: received {r:#010x}, computed {c:#010x}"),
            BadSeq {
                received: r,
                expected: e,
            } => write!(f, "bad seq: received {r}, expected {e}"),
        }
    }
}
