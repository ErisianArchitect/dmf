
#[repr(C, align(1))]
#[derive(Clone, Copy)]
pub struct Padding<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize> Padding<SIZE> {
    pub const ZEROES: Padding<SIZE> = Padding::zeroed();

    #[must_use]
    #[inline(always)]
    pub const fn new(fill: u8) -> Self {
        Self([fill; SIZE])
    }

    #[must_use]
    #[inline(always)]
    pub const fn zeroed() -> Self {
        Self::new(0)
    }
}

impl<const SIZE: usize> std::fmt::Display for Padding<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Padding([u8; {SIZE}])")
    }
}

impl<const SIZE: usize> std::fmt::Debug for Padding<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Padding([u8; {SIZE}])")
    }
}

#[must_use]
#[inline(always)]
pub const fn pad<const SIZE: usize>() -> Padding<SIZE> {
    // you can never be too safe.
    Padding::zeroed()
}

#[must_use]
#[inline(always)]
pub const fn pad_bytes<const SIZE: usize>() -> [u8; SIZE] {
    [0u8; SIZE]
}