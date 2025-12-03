use bytes::BytesMut;

use crate::reader;

pub struct Reserve;

impl reader::HandleBuffer for Reserve {
    fn required(&mut self, buffer: &mut BytesMut, length: usize) {
        buffer.reserve(buffer.capacity() - length);
    }
}
