
impl crate::Sealed<crate::SealType<bool>> for bool {}

pub trait BoolExt: crate::Sealed<crate::SealType<bool>> {
    fn select<T>(self, _true: T, _false: T) -> T;
    fn select_fn<T, TF: FnOnce() -> T, FF: FnOnce() -> T>(self, true_: TF, false_: FF) -> T;
    fn select_unary<T, V, TF: FnOnce(V) -> T, FF: FnOnce(V) -> T>(self, value: V, true_: TF, false_: FF) -> T;
    fn toggle(&mut self) -> Self;
    fn toggle_if(&mut self, condition: bool) -> Self;
    fn mark(&mut self) -> bool;
    fn mark_if(&mut self, condition: bool) -> bool;
    fn unmark(&mut self) -> bool;
    fn unmark_if(&mut self, condition: bool) -> bool;
    fn if_<F: FnOnce()>(self, if_: F);
    fn if_not<F: FnOnce()>(self, not_: F);
    fn if_else<R, If: FnOnce() -> R, Else: FnOnce() -> R>(self, if_: If, else_: Else) -> R;
}

impl BoolExt for bool {
    #[inline(always)]
    fn select<T>(self, true_: T, false_: T) -> T {
        if self {
            true_
        } else {
            false_
        }
    }

    /// Execute and return the value of _false or _true depending if self is false or true.
    #[inline]
    fn select_fn<T, TF: FnOnce() -> T, FF: FnOnce() -> T>(self, true_: TF, false_: FF) -> T {
        if self {
            true_()
        } else {
            false_()
        }
    }

    #[inline]
    fn select_unary<T, V, TF: FnOnce(V) -> T, FF: FnOnce(V) -> T>(self, value: V, true_: TF, false_: FF) -> T {
        if self {
            true_(value)
        } else {
            false_(value)
        }
    }

    #[inline(always)]
    fn toggle(&mut self) -> Self {
        *self ^= true;
        *self
    }

    #[inline(always)]
    fn toggle_if(&mut self, condition: bool) -> Self {
        *self ^= condition;
        *self
    }

    /// Sets value to true and returns true if it was previously false.
    #[inline]
    fn mark(&mut self) -> bool {
        !std::mem::replace(self, true)
    }

    /// If the condition is met, sets the value to true and returns true if the value was changed.
    #[inline]
    fn mark_if(&mut self, condition: bool) -> bool {
        if condition {
            let changed = !*self;
            *self = true;
            changed
        } else {
            false
        }
    }

    /// Sets value to false and returns true if it was previously true.
    #[inline]
    fn unmark(&mut self) -> bool {
        let changed = *self;
        *self = false;
        changed
    }

    /// If the condition is met, sets the value to false and returns true if the value was changed.
    #[inline]
    fn unmark_if(&mut self, condition: bool) -> bool {
        if condition {
            let changed = *self;
            *self = false;
            changed
        } else {
            false
        }
    }

    /// `if self { _if() }`
    #[inline]
    fn if_<F: FnOnce()>(self, if_: F) {
        if self { if_() }
    }
    
    /// `if !self { _not() }`
    #[inline]
    fn if_not<F: FnOnce()>(self, not: F) {
        if !self { not() }
    }

    /// Like `if-else`, but with closures!
    #[inline]
    fn if_else<R, If: FnOnce() -> R, Else: FnOnce() -> R>(self, if_: If, else_: Else) -> R {
        self.select_fn(if_, else_)
    }
}