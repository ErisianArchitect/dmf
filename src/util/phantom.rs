use std::marker::PhantomData;


pub type InvariantLifetime<'a> = std::marker::PhantomData<fn(&'a ())>;
pub type CovariantLifetime<'a> = std::marker::PhantomData<&'a ()>;

/// Creates a phantom lifetime type (`std::marker::PhantomData<&'a ()>` or `std::marker::PhantomData<fn(&'a ())>`)
/// # Example
/// ```rust, no_run
/// struct PhantomLife<'short, 'long> {
///     //                    lifetime is covariant (can be shortened)
///     _covariant_lifetime1: phantom_lifetime!('short), // covariant is default
///     _covariant_lifetime2: phantom_lifetime!(covariant: 'short),
///     //                    lifetime is invariant (must be exact)
///     _invariant_lifetime1: phatom_lifetime!(invariant: 'long),
/// }
/// ```
#[macro_export]
macro_rules! phantom_lifetime {
    // invariant - type must life exactly as long as lifetimes.
    (invariant: $($life:lifetime),+$(,)?) => {
        std::marker::PhantomData::<fn($(
            &$life (),
        )*)>
    };
    // covariant - type can live shorter lifetimes.
    (covariant: $($life:lifetime),+$(,)?) => {
        std::marker::PhantomData::<($(
            &$life (),
        )*)>
    };
    // default is covariant
    ($($life:lifetime),+$(,)?) => {
        $crate::phantom_lifetime!(covariant: $($life),*)
    };
    // With no tokens, we'll initialize PhantomData
    () => {
        std::marker::PhantomData
    };
}

pub use crate::phantom_lifetime;