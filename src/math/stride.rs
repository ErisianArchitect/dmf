
#[inline]
pub const fn stride_count(index: usize, stride: usize) -> usize {
    (index + stride - 1) / stride
}

pub trait StrideCount {
    /// The number of sections of `stride` width that can hold `length` elements.
    fn stride_count(self, stride: Self) -> Self;
}

macro_rules! stride_count_impls {
    ($type:ty) => {
        impl StrideCount for $type {
            #[inline]
            fn stride_count(self, stride: Self) -> Self {
                debug_assert_ne!(stride, 0, "stride is zero; causes division by zero.");
                (self + stride - 1) / stride
            }
        }
    };
}

crate::for_each_int_type!(stride_count_impls; unsigned);

pub trait Stride {
    /// The stride index.
    fn stride(self, stride: Self) -> Self;
}

macro_rules! stride_impls {
    ($type:ty) => {
        impl Stride for $type {
            #[inline]
            fn stride(self, stride: Self) -> Self {
                debug_assert_ne!(stride, 0, "stride is zero; causes division by zero.");
                self / stride
            }
        }
    };
}

crate::for_each_int_type!(stride_impls; unsigned);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stride_count_test() {
        let sc = u64::stride_count(4, 8);
        assert_eq!(sc, 1);
        let sc = u64::stride_count(20, 8);
        assert_eq!(sc, 3);
        let sc = u64::stride_count(64, 64);
        assert_eq!(sc, 1);
        let sc = u64::stride_count(65, 64);
        assert_eq!(sc, 2);
        let sc = u64::stride_count(0, 64);
        assert_eq!(sc, 0);
        let sc = u64::stride_count(1, 64);
        assert_eq!(sc, 1);
        let sc = 65u8.stride_count(64);
        assert_eq!(sc, 2);
        let length = 127usize;
        let sc = length.stride_count(64);
        assert_eq!(sc, 2);

        let si = 63u32.stride(64);
        assert_eq!(si, 0);
        let si = 64u32.stride(64);
        assert_eq!(si, 1);
        let si = 127u32.stride(64);
        assert_eq!(si, 1);
        let si = 128u32.stride(64);
        assert_eq!(si, 2);
    }
}