// C-compatible API for lexical conversion routines.

use crate::lib::slice;
use lexical_core;

// HELPERS

/// Calculate the difference between two pointers.
#[inline]
pub fn distance<T>(first: *const T, last: *const T)
    -> usize
{
    debug_assert!(last >= first, "range must be positive.");
    let f = first as usize;
    let l = last as usize;
    l - f
}

/// Convert a mutable pointer range to a mutable slice safely.
#[inline]
pub(crate) unsafe fn slice_from_range_mut<'a, T>(first: *mut T, last: *mut T)
    -> &'a mut [T]
{
    assert!(first <= last && !first.is_null() && !last.is_null());
    slice::from_raw_parts_mut(first, distance(first, last))
}

// FROM LEXICAL

/// Macro to generate complete parser from a pointer range.
macro_rules! lexical_from_range {
    (
        fn $name:ident,
        callback => $callback:ident,
        type => $type:ty,
        args => $($argname:ident : $argtype:ty ;)*,
        condition => $($condition:tt)*
    ) => (
        #[doc(hidden)]
        #[no_mangle]
        $($condition)*
        pub unsafe extern fn $name(first: *const u8, last: *const u8 $(,$argname : $argtype)*)
            -> $crate::result::Result<$type>
        {
            assert!(first <= last && !first.is_null() && !last.is_null());
            let bytes = $crate::lib::slice::from_raw_parts(first, distance(first, last));
            lexical_core::$callback(bytes $(,$argname)*).into()
        }
    );
}

// Macro to generate the partial parser from a pointer range.
macro_rules! lexical_partial_from_range {
    (
        fn $name:ident,
        callback => $callback:ident,
        type => $type:ty,
        args => $($argname:ident : $argtype:ty ;)*,
        condition => $($condition:tt)*
    ) => (
        #[doc(hidden)]
        #[no_mangle]
        $($condition)*
        pub unsafe extern fn $name(first: *const u8, last: *const u8 $(,$argname : $argtype)*)
            -> $crate::result::Result<$crate::result::Tuple<$type, usize>>
        {
            assert!(first <= last && !first.is_null() && !last.is_null());
            let bytes = $crate::lib::slice::from_raw_parts(first, distance(first, last));
            match lexical_core::$callback(bytes $(,$argname)*) {
                Ok(v)  => Ok(v.into()),
                Err(e) => Err(e),
            }.into()
        }
    );
}

// Macro to generate parsers implementing the FromLexical trait.
macro_rules! from_lexical {
    (
        type => $type:ty,
        decimal => $decimal_name:ident,
        partial_decimal => $partial_decimal_name:ident,
        options => $options_name:ident,
        partial_options => $partial_options_name:ident
    ) => (
        // Decimal.
        lexical_from_range!(
            fn $decimal_name,
            callback => parse,
            type => $type,
            args =>,
            condition =>
        );

        // Partial decimal.
        lexical_partial_from_range!(
            fn $partial_decimal_name,
            callback => parse_partial,
            type => $type,
            args =>,
            condition =>
        );

        // Options.
        lexical_from_range!(
            fn $options_name,
            callback => parse_with_options,
            type => $type,
            args => options: &<$type as lexical_core::FromLexical>::Options ;,
            condition =>
        );

        // Partial options.
        lexical_partial_from_range!(
            fn $partial_options_name,
            callback => parse_partial_with_options,
            type => $type,
            args => options: &<$type as lexical_core::FromLexical>::Options ;,
            condition =>
        );
    );
}

// TO LEXICAL

/// Macro to generate the lexical to_string API using a range.
macro_rules! lexical_to_range {
    (
        fn $name:ident,
        callback => $callback:ident,
        type => $type:ty,
        args => $($argname:ident : $argtype:ty)*,
        condition => $($condition:tt)*
    ) => (
        #[doc(hidden)]
        #[no_mangle]
        $($condition)*
        pub unsafe extern fn $name(value: $type $(,$argname : $argtype)* , first: *mut u8, last: *mut u8)
            -> *mut u8
        {
            let bytes = $crate::api::slice_from_range_mut(first, last);
            let slc = lexical_core::$callback(value, bytes $(,$argname)* );
            let len = slc.len();
            slc[len..].as_mut_ptr()
        }
    );
}

// Macro to generate serializers implementing the ToLexical trait.
macro_rules! to_lexical {
    (
        type => $type:ty,
        decimal => $decimal_name:ident,
        options => $options_name:ident
    ) => (
        // Decimal
        lexical_to_range!(
            fn $decimal_name,
            callback => write,
            type => $type,
            args =>,
            condition =>
        );

        // Options.
        lexical_to_range!(
            fn $options_name,
            callback => write_with_options,
            type => $type,
            args => options: &<$type as lexical_core::ToLexical>::Options,
            condition =>
        );
    );
}

// API

// ATOF
from_lexical!(
    type => f32,
    decimal => lexical_atof32,
    partial_decimal => lexical_atof32_partial,
    options => lexical_atof32_with_options,
    partial_options => lexical_atof32_partial_with_options
);
from_lexical!(
    type => f64,
    decimal => lexical_atof64,
    partial_decimal => lexical_atof64_partial,
    options => lexical_atof64_with_options,
    partial_options => lexical_atof64_partial_with_options
);

// ATOI
from_lexical!(
    type => u8,
    decimal => lexical_atou8,
    partial_decimal => lexical_atou8_partial,
    options => lexical_atou8_with_options,
    partial_options => lexical_atou8_partial_with_options
);
from_lexical!(
    type => u16,
    decimal => lexical_atou16,
    partial_decimal => lexical_atou16_partial,
    options => lexical_atou16_with_options,
    partial_options => lexical_atou16_partial_with_options
);
from_lexical!(
    type => u32,
    decimal => lexical_atou32,
    partial_decimal => lexical_atou32_partial,
    options => lexical_atou32_with_options,
    partial_options => lexical_atou32_partial_with_options
);
from_lexical!(
    type => u64,
    decimal => lexical_atou64,
    partial_decimal => lexical_atou64_partial,
    options => lexical_atou64_with_options,
    partial_options => lexical_atou64_partial_with_options
);
from_lexical!(
    type => usize,
    decimal => lexical_atousize,
    partial_decimal => lexical_atousize_partial,
    options => lexical_atousize_with_options,
    partial_options => lexical_atousize_partial_with_options
);
from_lexical!(
    type => u128,
    decimal => lexical_atou128,
    partial_decimal => lexical_atou128_partial,
    options => lexical_atou128_with_options,
    partial_options => lexical_atou128_partial_with_options
);

from_lexical!(
    type => i8,
    decimal => lexical_atoi8,
    partial_decimal => lexical_atoi8_partial,
    options => lexical_atoi8_with_options,
    partial_options => lexical_atoi8_partial_with_options
);
from_lexical!(
    type => i16,
    decimal => lexical_atoi16,
    partial_decimal => lexical_atoi16_partial,
    options => lexical_atoi16_with_options,
    partial_options => lexical_atoi16_partial_with_options
);
from_lexical!(
    type => i32,
    decimal => lexical_atoi32,
    partial_decimal => lexical_atoi32_partial,
    options => lexical_atoi32_with_options,
    partial_options => lexical_atoi32_partial_with_options
);
from_lexical!(
    type => i64,
    decimal => lexical_atoi64,
    partial_decimal => lexical_atoi64_partial,
    options => lexical_atoi64_with_options,
    partial_options => lexical_atoi64_partial_with_options
);
from_lexical!(
    type => isize,
    decimal => lexical_atoisize,
    partial_decimal => lexical_atoisize_partial,
    options => lexical_atoisize_with_options,
    partial_options => lexical_atoisize_partial_with_options
);
from_lexical!(
    type => i128,
    decimal => lexical_atoi128,
    partial_decimal => lexical_atoi128_partial,
    options => lexical_atoi128_with_options,
    partial_options => lexical_atoi128_partial_with_options
);

// FTOA
to_lexical!(
    type => f32,
    decimal => lexical_f32toa,
    options => lexical_f32toa_with_options
);
to_lexical!(
    type => f64,
    decimal => lexical_f64toa,
    options => lexical_f64toa_with_options
);

// ITOA
to_lexical!(
    type => u8,
    decimal => lexical_u8toa,
    options => lexical_u8toa_with_options
);
to_lexical!(
    type => u16,
    decimal => lexical_u16toa,
    options => lexical_u16toa_with_options
);
to_lexical!(
    type => u32,
    decimal => lexical_u32toa,
    options => lexical_u32toa_with_options
);
to_lexical!(
    type => u64,
    decimal => lexical_u64toa,
    options => lexical_u64toa_with_options
);
to_lexical!(
    type => usize,
    decimal => lexical_usizetoa,
    options => lexical_usizetoa_with_options
);
to_lexical!(
    type => u128,
    decimal => lexical_u128toa,
    options => lexical_u128toa_with_options
);

to_lexical!(
    type => i8,
    decimal => lexical_i8toa,
    options => lexical_i8toa_with_options
);
to_lexical!(
    type => i16,
    decimal => lexical_i16toa,
    options => lexical_i16toa_with_options
);
to_lexical!(
    type => i32,
    decimal => lexical_i32toa,
    options => lexical_i32toa_with_options
);
to_lexical!(
    type => i64,
    decimal => lexical_i64toa,
    options => lexical_i64toa_with_options
);
to_lexical!(
    type => isize,
    decimal => lexical_isizetoa,
    options => lexical_isizetoa_with_options
);
to_lexical!(
    type => i128,
    decimal => lexical_i128toa,
    options => lexical_i128toa_with_options
);
