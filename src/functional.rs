use paste::paste;

macro_rules! make_min_max {
    ($type:ident) => {
        paste!{
            #[doc = concat!("Returns the minimum [", stringify!($type), "] values.")]
            #[must_use]
            #[inline(always)]
            pub const fn [<min_ $type>](a: $type, b: $type) -> $type {
                $crate::select!(a <= b, a, b)
            }
            #[doc = concat!("Returns the maximum [", stringify!($type), "] values.")]
            #[must_use]
            #[inline(always)]
            pub const fn [<max_ $type>](a: $type, b: $type) -> $type {
                $crate::select!(a >= b, a, b)
            }
            #[doc = concat!("Returns the minimum and maximum (in that order) of the [", stringify!($type), "] values.")]
            #[must_use]
            #[inline(always)]
            pub const fn [<min_max_ $type>](a: $type, b: $type) -> ($type, $type) {
                $crate::select!(a <= b, (a, b), (b, a))
            }
        }
    };
    ($($type:ident),+$(,)?) => {
        $(
            make_min_max!($type);
        )*
    };
}

make_min_max!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
);

#[must_use]
#[inline(always)]
pub fn select<T>(condition: bool, true_: T, false_: T) -> T {
    crate::select!(condition, true_, false_)
}

#[must_use]
#[inline(always)]
pub fn min<T: PartialOrd<T>>(a: T, b: T) -> T {
    crate::min!(a, b)
}

#[must_use]
#[inline(always)]
pub fn max<T: PartialOrd<T>>(a: T, b: T) -> T {
    crate::max!(a, b)
}

#[must_use]
#[inline(always)]
pub fn min_max<T: PartialOrd<T>>(a: T, b: T) -> (T, T) {
    crate::min_max!(a, b)
}

#[inline(always)]
pub const fn half_f32(v: f32) -> f32 {
    crate::half!(v)
}

#[inline(always)]
pub const fn half_f64(v: f64) -> f64 {
    crate::half!(v)
}

#[inline(always)]
pub const fn quarter_f32(v: f32) -> f32 {
    crate::quarter!(v)
}

#[inline(always)]
pub const fn quarter_f64(v: f64) -> f64 {
    crate::quarter!(v)
}

#[inline(always)]
pub const fn tenth_f32(v: f32) -> f32 {
    crate::tenth!(v)
}

#[inline(always)]
pub const fn tenth_f64(v: f64) -> f64 {
    crate::tenth!(v)
}

#[inline(always)]
pub fn swap_pair<L, R>(pair: (L, R)) -> (R, L) {
    let (l, r) = pair;
    (r, l)
}

#[inline(always)]
pub const fn pass<T>(value: T) -> T {
    value
}

/// This functions just like `drop()`, but with a more clear name for instances where you want to ignore a value passed to a callback.
#[inline(always)]
pub fn ignore<T>(_: T) {}

#[inline(always)]
pub fn eval<R, F: FnOnce() -> R>(f: F) -> R {
    f()
}

#[inline(always)]
pub fn try_fn<T, E, F: FnOnce() -> Result<T, E>>(f: F) -> Result<T, E> {
    f()
}

/// This serves no real purpose. It does nothing. For no reason. Because why not?
/// I'm sure there's a use for this in an infinite universe.
#[inline(always)]
pub const fn noop() {}

macro_rules! make_noops {
    ($( $name:ident <$($t:ident),+$(,)?> ;)+) => {
        $(
            #[inline(always)]
            pub fn $name<$($t),*>($(_: $t),*) {}
        )+
    };
}

make_noops! {
    noop_1<T0>;
    noop_2<T0, T1>;
    noop_3<T0, T1, T2>;
    noop_4<T0, T1, T2, T3>;
    noop_5<T0, T1, T2, T3, T4>;
    noop_6<T0, T1, T2, T3, T4, T5>;
    noop_7<T0, T1, T2, T3, T4, T5, T6>;
    noop_8<T0, T1, T2, T3, T4, T5, T6, T7>;
    noop_9<T0, T1, T2, T3, T4, T5, T6, T7, T8>;
    noop_10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>;
    noop_11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>;
    noop_12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>;
    noop_13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>;
    noop_14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>;
    noop_15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>;
    noop_16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>;
    noop_17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>;
    noop_18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>;
    noop_19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>;
    noop_20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19>;
    noop_21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>;
    noop_22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21>;
    noop_23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22>;
    noop_24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23>;
    noop_25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24>;
    noop_26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25>;
    noop_27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26>;
    noop_28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27>;
    noop_29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28>;
    noop_30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29>;
    noop_31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30>;
    noop_32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31>;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn min_max_tests() {
        assert_eq!(min_u8(1, 2), 1);
        assert_eq!(min_u8(2, 1), 1);
        assert_eq!(max_u8(1, 2), 2);
        assert_eq!(max_u8(2, 1), 2);
        assert_eq!(min_f32(1.0, 2.0), 1.0);
        assert_eq!(min_f32(2.0, 1.0), 1.0);
        assert_eq!(max_f32(1.0, 2.0), 2.0);
        assert_eq!(max_f32(2.0, 1.0), 2.0);
        assert_eq!(min_max_u8(1, 2), (1, 2));
        assert_eq!(min_max_u8(2, 1), (1, 2));
        assert_eq!(min_max_f32(1.0, 2.0), (1.0, 2.0));
        assert_eq!(min_max_f32(2.0, 1.0), (1.0, 2.0));
    }
}