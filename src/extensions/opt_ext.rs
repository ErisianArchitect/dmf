
impl<T> crate::Sealed<crate::SealType<Option<T>>> for Option<T> {}

pub trait OptionExt<T>: crate::Sealed<crate::SealType<Option<T>>> {
    fn and_replace<F: FnOnce(T) -> Option<T>>(&mut self, replace: F);
    fn drop(&mut self);
    fn then<F: FnOnce(T)>(self, then: F);
    fn then_ref<F: FnOnce(&T)>(&self, then: F);
    fn then_mut<F: FnOnce(&mut T)>(&mut self, then: F);
    fn then_take<F: FnOnce(T)>(&mut self, then_take: F);
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    fn and_replace<F: FnOnce(T) -> Option<T>>(&mut self, replace: F) {
        if let Some(inner) = self.take() {
            *self = replace(inner);
        }
    }

    #[inline(always)]
    fn drop(&mut self) {
        *self = None;
    }

    #[inline]
    fn then<F: FnOnce(T)>(self, then: F) {
        if let Some(value) = self {
            then(value);
        }
    }

    #[inline]
    fn then_ref<F: FnOnce(&T)>(&self, then: F) {
        if let Some(value) = self {
            then(value);
        }
    }

    #[inline]
    fn then_mut<F: FnOnce(&mut T)>(&mut self, then: F) {
        if let Some(value) = self {
            then(value);
        }
    }

    #[inline]
    fn then_take<F: FnOnce(T)>(&mut self, then_take: F) {
        if let Some(value) = self.take() {
            then_take(value);
        }
    }
}