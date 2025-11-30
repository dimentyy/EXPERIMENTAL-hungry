use std::ptr;

use crate::tl::de::{Buf, Deserialize, Error};

impl Deserialize for Vec<u8> {
    const MINIMUM_SERIALIZED_LEN: usize = 4;

    #[inline]
    unsafe fn deserialize(buf: &mut Buf) -> Result<Self, Error> {
        unsafe {
            let len = *buf.ptr;

            let (ptr, len) = if len <= 253 {
                let len = len as usize;

                (buf.advance((len + 4) & !3)?.add(1), len)
            } else {
                let len = (u32::from_le((buf.ptr as *const u32).read_unaligned()) >> 8) as usize;

                (buf.advance((len + 7) & !3)?.add(4), len)
            };

            let mut vec = Vec::with_capacity(len);
            ptr::copy_nonoverlapping(ptr, vec.as_mut_ptr(), len);
            vec.set_len(len);
            Ok(vec)
        }
    }
}

impl Deserialize for String {
    const MINIMUM_SERIALIZED_LEN: usize = Vec::<u8>::MINIMUM_SERIALIZED_LEN;

    #[inline]
    unsafe fn deserialize(buf: &mut Buf) -> Result<Self, Error> {
        Ok(String::from_utf8(unsafe { Vec::<u8>::deserialize(buf)? })?)
    }
}
