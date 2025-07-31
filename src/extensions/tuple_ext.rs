use crate::SealType;


pub trait TuplePairExt<L, R>: crate::Sealed<SealType<(L, R)>> {
    fn swap_pair(self) -> (R, L);
}

impl<L, R> crate::Sealed<SealType<(L, R)>> for (L, R) {}

impl<L, R> TuplePairExt<L, R> for (L, R) {
    fn swap_pair(self) -> (R, L) {
        (self.1, self.0)
    }
}