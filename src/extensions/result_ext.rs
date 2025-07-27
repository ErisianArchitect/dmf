
impl<T, E> crate::Sealed<crate::SealType<Result<T, E>>> for Result<T, E> {}

pub trait ResultExt<T, E>: crate::Sealed<crate::SealType<Result<T, E>>> {
    type Ok;
    type Error;
    fn handle_err<F: FnOnce(Self::Error)>(self, f: F);
    fn try_fn<F: FnOnce() -> Self>(f: F) -> Self;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    type Ok = T;
    type Error = E;

    #[inline]
    fn handle_err<F: FnOnce(Self::Error)>(self, f: F) {
        if let Err(err) = self {
            f(err);
        }
    }

    /// This doesn't do anything special, it simply calls the closure passed to it. But it does make it easier to write
    /// fallible blocks.
    #[inline]
    fn try_fn<F: FnOnce() -> Self>(f: F) -> Self {
        f()
    }
}