const EXPECT_MSG: &str = "slice to be exact length";

pub(crate) trait SliceExt<T> {
    fn arr<const N: usize>(&self) -> &[T; N];
    fn arr_mut<const N: usize>(&mut self) -> &mut [T; N];
}

impl<T> SliceExt<T> for [T] {
    fn arr<const N: usize>(&self) -> &[T; N] {
        <&[T; N]>::try_from(self).expect(EXPECT_MSG)
    }

    fn arr_mut<const N: usize>(&mut self) -> &mut [T; N] {
        <&mut [T; N]>::try_from(self).expect(EXPECT_MSG)
    }
}
