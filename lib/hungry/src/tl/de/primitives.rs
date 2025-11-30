use crate::tl::de::{DeserializeInfallible, DeserializeUnchecked, Error};
use crate::tl::{BOOL_FALSE, BOOL_TRUE};

macro_rules! int {
    ( $( $typ:ty ),+ ) => { $(
        impl DeserializeInfallible for $typ {
            #[inline]
            unsafe fn deserialize_infallible(buf: *const u8) -> Self {
                Self::from_le(unsafe { (buf as *const Self).read_unaligned() })
            }
        }
    )+ }
}

int!(u32, i32, i64);

impl DeserializeInfallible for f64 {
    #[inline]
    unsafe fn deserialize_infallible(buf: *const u8) -> Self {
        Self::from_le_bytes(unsafe { (buf as *const [u8; 8]).read_unaligned() })
    }
}

impl DeserializeUnchecked for bool {
    #[inline]
    unsafe fn deserialize_unchecked(buf: *const u8) -> Result<Self, Error> {
        match unsafe { u32::deserialize_infallible(buf) } {
            BOOL_TRUE => Ok(true),
            BOOL_FALSE => Ok(false),
            id => Err(Error::UnexpectedConstructor { id }),
        }
    }
}
