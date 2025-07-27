
#[macro_export]
macro_rules! for_each_int_type {
    (@map:fixed unsigned; $macro:path) => {
        $macro!(u8);
        $macro!(u16);
        $macro!(u32);
        $macro!(u64);
        $macro!(u128);
    };
    (@map:fixed signed; $macro:path) => {
        $macro!(i8);
        $macro!(i16);
        $macro!(i32);
        $macro!(i64);
        $macro!(i128);
    };
    (@map:fixed; $macro:path) => {
        $crate::for_each_int_type!(@map: fixed unsigned; $macro);
        $crate::for_each_int_type!(@map: fixed signed; $macro);
    };
    (@map:pointer unsigned; $macro:path) => {
        $macro!(usize);
    };
    (@map:pointer signed; $macro:path) => {
        $macro!(isize);
    };
    (@map:pointer; $macro:path) => {
        $crate::for_each_int_type!(@map: pointer unsigned; $macro);
        $crate::for_each_int_type!(@map: pointer signed; $macro);
    };
    (@map:unsigned; $macro:path) => {
        $crate::for_each_int_type!(@map: fixed unsigned; $macro);
        $crate::for_each_int_type!(@map: pointer unsigned; $macro);
    };
    (@map:signed; $macro:path) => {
        $crate::for_each_int_type!(@map: fixed signed; $macro);
        $crate::for_each_int_type!(@map: pointer signed; $macro);
    };
    (@map:$any:ty; $macro:path) => {
        $macro!($any);
    };
    ($macro:path; $($flag:ident $($modifier:ident)?),+$(,)?) => {
        $(
            $crate::for_each_int_type!(@map: $flag $($modifier)?; $macro);
        )*
    };
    ($macro:path) => {
        $macro!(u8);
        $macro!(u16);
        $macro!(u32);
        $macro!(u64);
        $macro!(u128);
        $macro!(usize);
        $macro!(i8);
        $macro!(i16);
        $macro!(i32);
        $macro!(i64);
        $macro!(i128);
        $macro!(isize);
    };
    (@for $macro:path; $($type:ty)+) => {
        $(
            $macro!{$type}
        )*
    };
    () => {
        u8
        u16
        u32
        u64
        u128
        usize
        i8
        i16
        i32
        i64
        i128
        isize
    };
    (fixed unsigned) => {
        u8 u16 u32 u64 u128
    };
    (fixed signed) => {
        i8 i16 i32 i64 i128
    };
    (fixed) => {
        u8 u16 u32 u64 u128
        i8 i16 i32 i64 i128
    };
    (pointer unsigned) => {
        usize
    };
    (pointer signed) => {
        isize
    };
    (pointer) => {
        usize isize
    };
    (unsigned) => {
        u8 u16 u32 u64 u128 usize
    };
    (signed) => {
        i8 i16 i32 i64 i128 isize
    };
}

#[macro_export]
macro_rules! with_each_int_type {
    () => {
        
    };
}

#[macro_export]
macro_rules! for_each {
    ($macro:path:tt; $($token:tt)+) => {
        $(
            $macro!{$token}
        )*
    };
    ($macro:path:tt,; $($token:tt),+$(,)?) => {
        $(
            $macro!{$token}
        )*
    };
    ($macro:path:expr; $($token:expr),+) => {
        $(
            $macro!{$token}
        )*
    };
}

#[macro_export]
macro_rules! int_types {
    () => {
        u8
        u16
        u32
        u64
        u128
        usize
        i8
        i16
        i32
        i64
        i128
        isize
    };
    (fixed unsigned) => {
        u8 u16 u32 u64 u128
    };
    (fixed signed) => {
        i8 i16 i32 i64 i128
    };
    (fixed) => {
        u8 u16 u32 u64 u128
        i8 i16 i32 i64 i128
    };
    (pointer unsigned) => {
        usize
    };
    (pointer signed) => {
        isize
    };
    (pointer) => {
        usize isize
    };
    (unsigned) => {
        u8 u16 u32 u64 u128 usize
    };
    (signed) => {
        i8 i16 i32 i64 i128 isize
    };
    ($($flag:ident $($modifier:ident)?),+$(,)?) => {
        $(
            $crate::int_types!{$flag $($modifier)?}
        )*
    };
}

/// Breaks from the loop on ok with the inner value, otherwise returns the error.
#[macro_export]
macro_rules! break_ok {
    ($($label:lifetime)? $result:expr) => {
        $crate::break_ok!($($label)? $result, ok => ok)
    };
    ($($label:lifetime)? $result:expr, $ok_name:ident => $map:expr) => {
        match $result {
            Ok($ok_name) => {
                break $($label)? $map;
            }
            Err(err) => err
        }
    };
}

#[macro_export]
macro_rules! break_err {
    ($($label:lifetime)? $result:expr) => {
        $crate::break_err!($($label)? $result, err => Err(err))
    };
    ($($label:lifetime)? $result:expr, $err_name:ident => $map:expr) => {
        match $result {
            Ok(ok) => ok,
            Err($err_name) => break $($label)? $map,
        }
    };
}

/// Useful for when you want to ignore and continue on error but want to return on result.
#[macro_export]
macro_rules! return_ok {
    ($result:expr) => {
        $crate::return_ok!($result, ok => ok)
    };
    ($result:expr, $ok_name:ident => $map:expr) => {
        match $result {
            Ok($ok_name) => return $map,
            Err(err) => err
        }
    };
}

#[macro_export]
macro_rules! return_err {
    ($result:expr) => {
        $crate::return_err!($result, err => err)
    };
    ($result:expr, $err_name:ident => $map:expr) => {
        match $result {
            Ok(ok) => ok,
            Err($err_name) => return $map,
        }
    };
}

#[macro_export]
macro_rules! break_some {
    ($($label:lifetime)? $option:expr) => {
        $crate::break_some!($($label)? $option, some => some);
    };
    ($($label:lifetime)? $option:expr, $some_name:ident => $map:expr) => {
        if let Some($some_name) = $option {
            break $($label)? $map;
        }
    };
}

#[macro_export]
macro_rules! break_none {
    ($($label:lifetime)? $option:expr $(, $result:expr)?) => {
        match $option {
            Some(some) => some,
            None => break $($label)? $($result)?,
        }
    };
}

#[macro_export]
macro_rules! return_some {
    ($option:expr) => {
        $crate::return_some!($option, some => some);
    };
    ($option:expr, $some_name:ident => $map:expr) => {
        if let Some($some_name) = $option {
            return $map;
        }
    };
}

#[macro_export]
macro_rules! return_none {
    ($option:expr $(, $result:expr)?) => {
        match $option {
            Some(some) => some,
            None => return $($result)?,
        }
    };
}

#[macro_export]
macro_rules! continue_ok {
    ($($label:lifetime)? $result:expr) => {
        if ($result).is_ok() {
            continue $($label)?;
        }
    };
}

#[macro_export]
macro_rules! continue_err {
    ($($label:lifetime)? $result:expr) => {
        if ($result).is_err() {
            continue $($label)?;
        }
    };
}

#[macro_export]
macro_rules! continue_some {
    ($($label:lifetime)? $result:expr) => {
        if ($result).is_some() {
            continue $($label)?;
        }
    };
}

#[macro_export]
macro_rules! continue_none {
    ($($label:lifetime)? $result:expr) => {
        if ($result).is_none() {
            continue $($label)?;
        }
    };
}

#[macro_export]
macro_rules! break_if {
    ($($label:lifetime)? $condition:expr $(, $value:expr)?) => {
        if ($condition) {
            break $($label)? $($value)?;
        }
    };
}

#[macro_export]
macro_rules! return_if {
    ($condition:expr $(, $value:expr)?) => {
        if ($condition) {
            return ($value);
        }
    };
}

#[macro_export]
macro_rules! continue_if {
    ($($label:lifetime)? $condition:expr) => {
        if ($condition) {
            continue $($label)?;
        }
    };
}

#[macro_export]
macro_rules! do_while {
    ($($label:lifetime : )? do $do_block:block while $condition:expr $(; /* allow for trailing semi-colon */)?) => {
        $($label : )? loop {
            $do_block
            if !($condition) {
                break;
            }
        }
    };
}

/// This macro is merely used in developer contexts where Rust-Analyzer or something similar can be used to
/// hover the mouse over the given name in order to determine the size of the type.
#[macro_export]
macro_rules! type_size_hint {
    ($anchor:ident for $type:ty) => {
        // adding the const declaration inside of this const block prevents
        // namespace pollution.
        const _: () = {
            #[allow(unused)]
            #[allow(non_upper_case_globals)]
            const $anchor: usize = std::mem::size_of::<$type>();
        };
    };
    ($type_ident:ident) => {
        $crate::type_size_hint!($type_ident for $type_ident);
    };
}

#[macro_export]
macro_rules! option_size_optimization {
    ($anchor:ident for $type:ty) => {
        // adding the const declaration inside of this const block prevents
        // namespace pollution.
        const _: () = {
            #[allow(unused)]
            #[allow(non_upper_case_globals)]
            const $anchor: bool = std::mem::size_of::<$type>() == std::mem::size_of::<Option<$type>>();
        };
    };
    ($type_ident:ident) => {
        $crate::option_size_optimization!($type_ident for $type_ident);
    };
}

#[inline(always)]
const fn max_usize(lhs: usize, rhs: usize) -> usize {
    if lhs >= rhs {
        lhs
    } else {
        rhs
    }
}

#[macro_export]
macro_rules! result_size_optimization {
    ($anchor:ident for $ok:ty, $err:ty) => {
        // adding the const declaration inside of this const block prevents
        // namespace pollution.
        const _: () = {
            #[allow(unused)]
            #[allow(non_upper_case_globals)]
            const $anchor: bool = max_usize(std::mem::size_of::<$ok>(), std::mem::size_of::<$err>()) <= std::mem::size_of::<Result<$ok, $err>>();
        };
    };
}

type_size_hint!(hint for Option<&str>);
option_size_optimization!(hint for &str);
result_size_optimization!(hint for &str, &str);

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused)]
    #[allow(non_upper_case_globals)]
    const snake: usize = 1;

    #[test]
    fn do_while_test() {
        let mut n = 0;
        #[allow(unused_assignments)]
        let mut assigned = false;
        do_while!(
            'test_loop: do {
                assert_eq!(n, 0);
                assigned = true;
                n += 1;
                if n == 3 {
                    break 'test_loop;
                }
            } while n > 100;
        );
        assert_eq!(n, 1);
        assert!(assigned);
    }
    
    #[test]
    fn ret_ok_test() {
        fn ok_on_n(n: i32, input: i32) -> Result<i32, i32> {
            if n == input {
                Ok(n)
            } else {
                Err(input)
            }
        }
        fn returns_result() -> i32 {
            let mut counter = 0;
            let result = loop {
                let current = break_ok!(ok_on_n(7, counter), ok => Result::<i32, String>::Ok(ok));
                assert_eq!(current, counter);
                counter += 1;
            };
            let _failure = return_ok!(result);
            panic!("Should not run.");
        }
        assert_eq!(returns_result(), 7);
    }
}