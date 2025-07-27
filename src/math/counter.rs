
type CM<T> = Counter<T>;

/// Incrementation of integer (or even non-integer) types.
/// These are functions that act on a mutable reference to self
/// for the purpose of incrementation.
pub trait Increment {
    type Result;
    /// Increments the value.
    fn increment(&mut self);
    /// Increment and return the result of incrementation.
    fn pre_increment(&mut self) -> Self::Result;
    /// Increment and return the value before incrementation.
    fn post_increment(&mut self) -> Self::Result;
}

pub trait Decrement {
    type Result;
    /// Decrements the value.
    fn decrement(&mut self);
    /// Decrement and return the result of decrementation.
    fn pre_decrement(&mut self) -> Self::Result;
    /// Decrement and return the value before decrementation.
    fn post_decrement(&mut self) -> Self::Result;
}

/// 
#[repr(transparent)]
#[derive(Debug)]
pub struct Counter<T: crate::Sealed<CM<T>>>(T);

impl<T: Clone + crate::Sealed<CM<T>>> Clone for Counter<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.0 = source.0.clone()
    }
}

impl<T: Copy + crate::Sealed<CM<T>>> Copy for Counter<T> {}

impl<T: PartialEq + crate::Sealed<CM<T>>> PartialEq for Counter<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl<T: Eq + crate::Sealed<CM<T>>> Eq for Counter<T> {}

impl<T: PartialOrd + crate::Sealed<CM<T>>> PartialOrd for Counter<T> {
    fn ge(&self, other: &Self) -> bool {
        self.0 >= other.0
    }

    fn gt(&self, other: &Self) -> bool {
        self.0 > other.0
    }

    fn le(&self, other: &Self) -> bool {
        self.0 <= other.0
    }

    fn lt(&self, other: &Self) -> bool {
        self.0 < other.0
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Ord + crate::Sealed<CM<T>>> Ord for Counter<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }

    fn clamp(self, min: Self, max: Self) -> Self
        where
            Self: Sized, {
        Self(self.0.clamp(min.0, max.0))
    }

    fn max(self, other: Self) -> Self
        where
            Self: Sized, {
        if self.0 >= other.0 {
            self
        } else {
            other
        }
    }

    fn min(self, other: Self) -> Self
        where
            Self: Sized, {
        if self.0 <= other.0 {
            self
        } else {
            other
        }
    }
}

impl<T: std::hash::Hash + crate::Sealed<CM<T>>> std::hash::Hash for Counter<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }

    fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
        where
            Self: Sized, {
        unsafe {
            T::hash_slice(std::mem::transmute(data), state);
        }
    }
}

impl<T: std::fmt::Display + crate::Sealed<CM<T>>> std::fmt::Display for Counter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "Counter({:#})", self.0)
        } else {
            write!(f, "Counter({})", self.0)
        }
    }
}

#[inline]
pub const fn counter<T: crate::Sealed<CM<T>>>(initial: T) -> Counter<T> {
    Counter(initial)
}

impl<T: crate::Sealed<CM<T>>> Counter<T> {
    #[inline]
    pub const fn new(init: T) -> Self {
        Self(init)
    }

    #[inline]
    pub fn inner(self) -> T {
        self.0
    }
}

macro_rules! counter_impls {
    ($type:ty) => {
        impl crate::Sealed<CM<$type>> for $type {}
        impl Counter<$type> {
            #[doc = "Increments the value."]
            #[inline(always)]
            pub const fn increment(&mut self) {
                self.0 += 1;
            }

            #[doc = "Pre-increments value before returning it."]
            #[inline(always)]
            pub const fn pre_increment(&mut self) -> $type {
                self.increment();
                self.0
            }

            #[doc = "Increments and returns the value prior to incrementation."]
            #[inline(always)]
            pub const fn post_increment(&mut self) -> $type {
                let result = self.0;
                self.increment();
                result
            }

            #[doc = "Decrements the value."]
            #[inline(always)]
            pub const fn decrement(&mut self) {
                self.0 -= 1;
            }
            
            #[doc = "Pre-decrements value before returning it."]
            #[inline(always)]
            pub const fn pre_decrement(&mut self) -> $type {
                self.decrement();
                self.0
            }
            
            #[doc = "Decrements and returns the value prior to decrementation."]
            #[inline(always)]
            pub const fn post_decrement(&mut self) -> $type {
                let result = self.0;
                self.decrement();
                result
            }
        }

        impl Increment for Counter<$type> {
            type Result = $type;
            #[doc = "Increments the value."]
            #[inline(always)]
            fn increment(&mut self) {
                Counter::<$type>::increment(self)
            }

            #[doc = "Pre-increments value before returning it."]
            #[inline(always)]
            fn pre_increment(&mut self) -> $type {
                Counter::<$type>::pre_increment(self)
            }

            #[doc = "Increments and returns the value prior to incrementation."]
            #[inline(always)]
            fn post_increment(&mut self) -> $type {
                Counter::<$type>::post_increment(self)
            }
        }

        impl Decrement for Counter<$type> {
            type Result = $type;

            #[doc = "Decrements the value."]
            #[inline(always)]
            fn decrement(&mut self) {
                Counter::<$type>::decrement(self);
            }

            #[doc = "Pre-decrements value before returning it."]
            #[inline(always)]
            fn pre_decrement(&mut self) -> $type {
                Counter::<$type>::pre_decrement(self)
            }

            #[doc = "Decrements and returns the value prior to decrementation."]
            #[inline(always)]
            fn post_decrement(&mut self) -> $type {
                Counter::<$type>::post_decrement(self)
            }
        }

        impl Increment for $type {
            type Result = $type;

            #[doc = "Increments the value."]
            #[inline(always)]
            fn increment(&mut self) {
                *self += 1;
            }

            #[doc = "Pre-increments value before returning it."]
            #[inline(always)]
            fn pre_increment(&mut self) -> $type {
                *self += 1;
                *self
            }

            #[doc = "Increments and returns the value prior to incrementation."]
            #[inline(always)]
            fn post_increment(&mut self) -> $type {
                let result = *self;
                *self += 1;
                result
            }
        }

        impl Decrement for $type {
            type Result = $type;
            #[doc = "Decrements the value."]
            #[inline(always)]
            fn decrement(&mut self) {
                *self -= 1;
            }

            #[doc = "Pre-decrements value before returning it."]
            #[inline(always)]
            fn pre_decrement(&mut self) -> $type {
                *self -= 1;
                *self
            }

            #[doc = "Decrements and returns the value prior to decrementation."]
            #[inline(always)]
            fn post_decrement(&mut self) -> $type {
                let result = *self;
                *self -= 1;
                result
            }
        }

        impl From<$type> for Counter<$type> {
            #[inline(always)]
            fn from(value: $type) -> Self {
                Self(value)
            }
        }

        impl From<Counter<$type>> for $type {
            #[inline(always)]
            fn from(value: Counter<$type>) -> Self {
                value.0
            }
        }
    };
}

crate::for_each_int_type!(counter_impls);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_tests() {
        let mut count = counter(0u32);

        let value = count.post_increment();

        assert_eq!(value, 0u32);
        assert_eq!(count, counter(1u32));

        let value = count.post_decrement();

        assert_eq!(value, 1u32);
        assert_eq!(count, counter(0u32));

        let value = count.pre_increment();

        assert_eq!(value, 1u32);
        assert_eq!(count, counter(1u32));
        
        let value = count.pre_decrement();
        
        assert_eq!(value, 0u32);
        assert_eq!(count, counter(0u32));

        let mut count = 0u32;

        let value = count.pre_increment();

        assert_eq!(value, 1u32);
        assert_eq!(count, 1u32);

        let value = count.pre_decrement();

        assert_eq!(value, 0u32);
        assert_eq!(count, 0u32);

        let value = count.post_increment();

        assert_eq!(value, 0u32);
        assert_eq!(count, 1u32);

        let value = count.post_decrement();

        assert_eq!(value, 1u32);
        assert_eq!(count, 0u32);

        let mut id = counter(0u32);
        
        let ids = [
            id.post_increment(),
            id.post_increment(),
            id.post_increment(),
            id.post_increment(),
            id.post_increment(),
        ];

        assert_eq!(ids, [0, 1, 2, 3, 4]);

    }
}