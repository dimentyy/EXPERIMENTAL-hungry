use std::ptr;

use crate::tl::Serialize;

impl Serialize for [u8] {
    #[inline]
    fn serialized_len(&self) -> usize {
        if self.len() <= 253 {
            (self.len() + 4) & !3
        } else {
            (self.len() + 7) & !3
        }
    }

    unsafe fn serialize_unchecked(&self, mut buf: *mut u8) -> *mut u8 {
        unsafe {
            if self.len() <= 253 {
                *buf = self.len() as u8;

                ptr::copy_nonoverlapping(self.as_ptr(), buf.add(1), self.len());

                if self.len() & 1 == 0 {
                    *buf.add(self.len() + 1) = 0;
                }

                if self.len() & 2 == 2 {
                    *(buf.add((self.len() & !1) + 2) as *mut u16) = 0;
                }

                return buf.add((self.len() & !3) + 4);
            }

            buf = (((self.len() as u32) << 8) | 254).serialize_unchecked(buf);

            ptr::copy_nonoverlapping(self.as_ptr(), buf, self.len());

            if self.len() | 0 & 1 == 1 {
                *buf.add(self.len()) = 0;
            }

            if self.len() & 2 == 2 {
                *(buf.add((self.len() + 1) & !1) as *mut u16) = 0;
            }

            buf.add((self.len() + 3) & !3usize)
        }
    }
}

impl Serialize for Vec<u8> {
    #[inline]
    fn serialized_len(&self) -> usize {
        self.as_slice().serialized_len()
    }

    #[inline]
    unsafe fn serialize_unchecked(&self, buf: *mut u8) -> *mut u8 {
        unsafe { self.as_slice().serialize_unchecked(buf) }
    }
}

impl Serialize for String {
    #[inline]
    fn serialized_len(&self) -> usize {
        self.as_bytes().serialized_len()
    }

    #[inline]
    unsafe fn serialize_unchecked(&self, buf: *mut u8) -> *mut u8 {
        unsafe { self.as_bytes().serialize_unchecked(buf) }
    }
}
