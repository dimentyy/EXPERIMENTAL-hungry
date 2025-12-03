use bytes::BytesMut;

use crate::reader;
use crate::transport::Unpack;

pub struct Split;

impl reader::HandleOutput for Split {
    type Output = (BytesMut, Unpack);

    fn acquired(&mut self, buffer: &mut BytesMut, unpack: Unpack) -> Self::Output {
        (buffer.split(), unpack)
    }
}
